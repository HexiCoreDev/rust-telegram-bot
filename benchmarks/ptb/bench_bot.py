"""PTB benchmark bot — identical features to Rust versions."""
import asyncio
import os
import logging

import uvicorn
from starlette.applications import Starlette
from starlette.requests import Request
from starlette.responses import PlainTextResponse, Response
from starlette.routing import Route

from telegram import Update, InlineKeyboardButton, InlineKeyboardMarkup
from telegram.constants import ChatAction
from telegram.ext import (
    Application,
    CommandHandler,
    CallbackQueryHandler,
    MessageHandler,
    ContextTypes,
    filters,
)

logging.basicConfig(level=logging.WARNING)

TOKEN = os.environ["TELEGRAM_BOT_TOKEN"]
URL = os.environ["WEBHOOK_URL"]


async def start(update: Update, context: ContextTypes.DEFAULT_TYPE):
    name = update.effective_user.first_name if update.effective_user else "there"
    keyboard = [
        [
            InlineKeyboardButton("Option 1", callback_data="1"),
            InlineKeyboardButton("Option 2", callback_data="2"),
        ],
        [InlineKeyboardButton("Option 3", callback_data="3")],
    ]
    await update.message.reply_text(
        f"Hi {name}! I am a benchmark bot.\nUse /help for info.",
        reply_markup=InlineKeyboardMarkup(keyboard),
    )


async def help_cmd(update: Update, context: ContextTypes.DEFAULT_TYPE):
    await update.message.reply_text(
        "Commands: /start, /help\n"
        "Send any text to echo.\n"
        "Press inline buttons to test callbacks."
    )


async def echo(update: Update, context: ContextTypes.DEFAULT_TYPE):
    await context.bot.send_chat_action(
        chat_id=update.effective_chat.id, action=ChatAction.TYPING
    )
    await update.message.reply_text(update.message.text)


async def button(update: Update, context: ContextTypes.DEFAULT_TYPE):
    query = update.callback_query
    await query.answer()
    await query.edit_message_text(text=f"You selected: Option {query.data}")


async def main():
    app = Application.builder().token(TOKEN).updater(None).build()
    app.add_handler(CommandHandler("start", start))
    app.add_handler(CommandHandler("help", help_cmd))
    app.add_handler(CallbackQueryHandler(button))
    app.add_handler(MessageHandler(filters.TEXT & ~filters.COMMAND, echo))

    await app.bot.set_webhook(
        url=f"{URL}/telegram", allowed_updates=Update.ALL_TYPES
    )

    async def telegram(request: Request):
        data = await request.json()
        await app.update_queue.put(
            Update.de_json(data=data, bot=app.bot)
        )
        return Response()

    async def health(_: Request):
        return PlainTextResponse("OK")

    starlette_app = Starlette(
        routes=[
            Route("/telegram", telegram, methods=["POST"]),
            Route("/healthcheck", health, methods=["GET"]),
        ]
    )
    server = uvicorn.Server(
        config=uvicorn.Config(
            app=starlette_app,
            port=8000,
            host="127.0.0.1",
            use_colors=False,
            log_level="warning",
        )
    )

    async with app:
        await app.start()
        print("PTB benchmark bot running on port 8000. Send /start to test.")
        await server.serve()
        await app.stop()


if __name__ == "__main__":
    asyncio.run(main())
