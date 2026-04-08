//! Stress tests for persistence backends under concurrent access.
//!
//! These tests verify that `DictPersistence` and `JsonFilePersistence` remain
//! consistent when many tasks write simultaneously.

use std::sync::Arc;

use serde_json::Value;
use tokio::sync::Barrier;

use telegram_bot_ext::persistence::base::BasePersistence;
use telegram_bot_ext::persistence::dict::DictPersistence;
use telegram_bot_ext::utils::types::JsonMap;

// ---------------------------------------------------------------------------
// DictPersistence stress tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn dict_concurrent_user_data_writes() {
    let persistence = Arc::new(DictPersistence::new());
    let task_count: i64 = 100;
    let barrier = Arc::new(Barrier::new(task_count as usize));

    let mut handles = Vec::with_capacity(task_count as usize);

    for i in 0..task_count {
        let p = Arc::clone(&persistence);
        let b = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            // Wait for all tasks to be ready before writing.
            b.wait().await;
            let mut data = JsonMap::new();
            data.insert("task".into(), Value::Number(i.into()));
            p.update_user_data(i, &data).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // Verify all writes were preserved.
    let user_data = persistence.get_user_data().await.unwrap();
    assert_eq!(
        user_data.len(),
        task_count as usize,
        "Expected {task_count} user entries, got {}",
        user_data.len()
    );

    for i in 0..task_count {
        let entry = user_data.get(&i).unwrap_or_else(|| panic!("Missing user_data for user {i}"));
        assert_eq!(
            entry.get("task"),
            Some(&Value::Number(i.into())),
            "Incorrect data for user {i}"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn dict_concurrent_chat_data_writes() {
    let persistence = Arc::new(DictPersistence::new());
    let task_count: i64 = 100;
    let barrier = Arc::new(Barrier::new(task_count as usize));

    let mut handles = Vec::with_capacity(task_count as usize);

    for i in 0..task_count {
        let p = Arc::clone(&persistence);
        let b = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            b.wait().await;
            let mut data = JsonMap::new();
            data.insert("chat_task".into(), Value::Number(i.into()));
            p.update_chat_data(i, &data).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let chat_data = persistence.get_chat_data().await.unwrap();
    assert_eq!(chat_data.len(), task_count as usize);

    for i in 0..task_count {
        let entry = chat_data.get(&i).unwrap();
        assert_eq!(entry.get("chat_task"), Some(&Value::Number(i.into())));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn dict_flush_after_concurrent_writes() {
    let persistence = Arc::new(DictPersistence::new());
    let task_count: i64 = 50;
    let barrier = Arc::new(Barrier::new(task_count as usize));

    let mut handles = Vec::with_capacity(task_count as usize);

    for i in 0..task_count {
        let p = Arc::clone(&persistence);
        let b = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            b.wait().await;
            let mut data = JsonMap::new();
            data.insert("val".into(), Value::Number(i.into()));
            p.update_user_data(i, &data).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // flush() on DictPersistence is a no-op, but it must not panic or corrupt
    // state when called after concurrent writes.
    persistence.flush().await.unwrap();

    // Verify data is still intact after flush.
    let user_data = persistence.get_user_data().await.unwrap();
    assert_eq!(user_data.len(), task_count as usize);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn dict_drop_user_data_under_concurrent_access() {
    let persistence = Arc::new(DictPersistence::new());

    // Pre-populate 50 users.
    for i in 0..50i64 {
        let mut data = JsonMap::new();
        data.insert("x".into(), Value::Number(i.into()));
        persistence.update_user_data(i, &data).await.unwrap();
    }

    let barrier = Arc::new(Barrier::new(50));
    let mut handles = Vec::with_capacity(50);

    // Concurrently drop all even-numbered users while writing to odd-numbered ones.
    for i in 0..50i64 {
        let p = Arc::clone(&persistence);
        let b = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            b.wait().await;
            if i % 2 == 0 {
                p.drop_user_data(i).await.unwrap();
            } else {
                let mut data = JsonMap::new();
                data.insert("updated".into(), Value::Bool(true));
                p.update_user_data(i, &data).await.unwrap();
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let user_data = persistence.get_user_data().await.unwrap();

    // All even users should be dropped.
    for i in (0..50i64).step_by(2) {
        assert!(
            !user_data.contains_key(&i),
            "User {i} should have been dropped"
        );
    }

    // All odd users should have the updated data.
    for i in (1..50i64).step_by(2) {
        let entry = user_data.get(&i).unwrap_or_else(|| panic!("Missing user {i}"));
        assert_eq!(entry.get("updated"), Some(&Value::Bool(true)));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn dict_drop_chat_data_under_concurrent_access() {
    let persistence = Arc::new(DictPersistence::new());

    // Pre-populate 50 chats.
    for i in 0..50i64 {
        let mut data = JsonMap::new();
        data.insert("y".into(), Value::Number(i.into()));
        persistence.update_chat_data(i, &data).await.unwrap();
    }

    let barrier = Arc::new(Barrier::new(50));
    let mut handles = Vec::with_capacity(50);

    for i in 0..50i64 {
        let p = Arc::clone(&persistence);
        let b = Arc::clone(&barrier);
        handles.push(tokio::spawn(async move {
            b.wait().await;
            if i % 2 == 0 {
                p.drop_chat_data(i).await.unwrap();
            } else {
                let mut data = JsonMap::new();
                data.insert("updated".into(), Value::Bool(true));
                p.update_chat_data(i, &data).await.unwrap();
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let chat_data = persistence.get_chat_data().await.unwrap();

    for i in (0..50i64).step_by(2) {
        assert!(!chat_data.contains_key(&i), "Chat {i} should have been dropped");
    }

    for i in (1..50i64).step_by(2) {
        let entry = chat_data.get(&i).unwrap_or_else(|| panic!("Missing chat {i}"));
        assert_eq!(entry.get("updated"), Some(&Value::Bool(true)));
    }
}

// ---------------------------------------------------------------------------
// JsonFilePersistence stress tests (feature-gated)
// ---------------------------------------------------------------------------

#[cfg(feature = "persistence-json")]
mod json_file_stress {
    use super::*;
    use telegram_bot_ext::persistence::json_file::JsonFilePersistence;

    /// Concurrent writes to in-memory state with `on_flush=true`, then a single
    /// `flush()` persists everything atomically.  This is the recommended usage
    /// pattern for `JsonFilePersistence` under concurrent access.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_file_concurrent_writes_then_flush() {
        let dir = std::env::temp_dir().join("tg_stress_json_rw");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("stress_data");

        // Use on_flush=true so concurrent writes go to memory only.
        let persistence = Arc::new(JsonFilePersistence::new(&base, true, true));
        let task_count: i64 = 50;
        let barrier = Arc::new(Barrier::new(task_count as usize));

        let mut handles = Vec::with_capacity(task_count as usize);

        for i in 0..task_count {
            let p = Arc::clone(&persistence);
            let b = Arc::clone(&barrier);
            handles.push(tokio::spawn(async move {
                b.wait().await;
                let mut data = JsonMap::new();
                data.insert("worker".into(), Value::Number(i.into()));
                p.update_user_data(i, &data).await.unwrap();
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // All writes should be in memory.
        let user_data = persistence.get_user_data().await.unwrap();
        assert_eq!(user_data.len(), task_count as usize);

        // File should not exist yet.
        assert!(!base.with_extension("json").exists());

        // Flush captures all data.
        persistence.flush().await.unwrap();
        assert!(base.with_extension("json").exists());

        // Re-open from disk and verify all entries.
        let p2 = JsonFilePersistence::new(&base, true, false);
        let reloaded = p2.get_user_data().await.unwrap();
        assert_eq!(reloaded.len(), task_count as usize);

        for i in 0..task_count {
            let entry = reloaded.get(&i).unwrap_or_else(|| panic!("Missing user {i}"));
            assert_eq!(entry.get("worker"), Some(&Value::Number(i.into())));
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Verify that the atomic write mechanism produces valid JSON even under
    /// concurrent in-memory writes followed by a flush.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_file_atomic_write_no_corruption() {
        let dir = std::env::temp_dir().join("tg_stress_json_atomic");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("atomic_data");

        let persistence = Arc::new(JsonFilePersistence::new(&base, true, true));
        let task_count: i64 = 30;
        let barrier = Arc::new(Barrier::new(task_count as usize));

        let mut handles = Vec::with_capacity(task_count as usize);

        for i in 0..task_count {
            let p = Arc::clone(&persistence);
            let b = Arc::clone(&barrier);
            handles.push(tokio::spawn(async move {
                b.wait().await;
                let mut data = JsonMap::new();
                data.insert("val".into(), Value::Number(i.into()));
                p.update_user_data(i, &data).await.unwrap();
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        persistence.flush().await.unwrap();

        // Re-open from disk and verify no corruption.
        let p2 = JsonFilePersistence::new(&base, true, false);
        let reloaded = p2.get_user_data().await.unwrap();

        assert_eq!(reloaded.len(), task_count as usize);

        // Verify each entry has correct structure.
        for (user_id, data) in &reloaded {
            assert!(
                data.contains_key("val"),
                "User {user_id} data is missing 'val' key"
            );
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Concurrent reads interleaved with a single writer -- verify that readers
    /// always see consistent state (either pre-write or post-write).
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_file_concurrent_read_write() {
        let dir = std::env::temp_dir().join("tg_stress_json_crw");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("crw_data");

        let persistence = Arc::new(JsonFilePersistence::new(&base, true, true));

        // Pre-populate some data.
        for i in 0..20i64 {
            let mut data = JsonMap::new();
            data.insert("init".into(), Value::Number(i.into()));
            persistence.update_user_data(i, &data).await.unwrap();
        }

        let barrier = Arc::new(Barrier::new(40));
        let mut handles = Vec::with_capacity(40);

        // 20 writers (adding new users)
        for i in 20..40i64 {
            let p = Arc::clone(&persistence);
            let b = Arc::clone(&barrier);
            handles.push(tokio::spawn(async move {
                b.wait().await;
                let mut data = JsonMap::new();
                data.insert("new".into(), Value::Number(i.into()));
                p.update_user_data(i, &data).await.unwrap();
            }));
        }

        // 20 readers (reading existing users)
        for _i in 0..20 {
            let p = Arc::clone(&persistence);
            let b = Arc::clone(&barrier);
            handles.push(tokio::spawn(async move {
                b.wait().await;
                let data = p.get_user_data().await.unwrap();
                // At minimum, the initial 20 entries should be present.
                assert!(data.len() >= 20);
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // After all tasks complete, we should have all 40 entries.
        let final_data = persistence.get_user_data().await.unwrap();
        assert_eq!(final_data.len(), 40);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_file_drop_user_data_concurrent() {
        let dir = std::env::temp_dir().join("tg_stress_json_drop_user");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("drop_user_data");

        let persistence = Arc::new(JsonFilePersistence::new(&base, true, true));

        // Pre-populate.
        for i in 0..40i64 {
            let mut data = JsonMap::new();
            data.insert("init".into(), Value::Number(i.into()));
            persistence.update_user_data(i, &data).await.unwrap();
        }

        let barrier = Arc::new(Barrier::new(40));
        let mut handles = Vec::with_capacity(40);

        for i in 0..40i64 {
            let p = Arc::clone(&persistence);
            let b = Arc::clone(&barrier);
            handles.push(tokio::spawn(async move {
                b.wait().await;
                if i % 2 == 0 {
                    p.drop_user_data(i).await.unwrap();
                } else {
                    let mut data = JsonMap::new();
                    data.insert("updated".into(), Value::Bool(true));
                    p.update_user_data(i, &data).await.unwrap();
                }
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let user_data = persistence.get_user_data().await.unwrap();

        for i in (0..40i64).step_by(2) {
            assert!(!user_data.contains_key(&i), "User {i} should have been dropped");
        }

        for i in (1..40i64).step_by(2) {
            let entry = user_data.get(&i).unwrap_or_else(|| panic!("Missing user {i}"));
            assert_eq!(entry.get("updated"), Some(&Value::Bool(true)));
        }

        // Flush and verify persistence.
        persistence.flush().await.unwrap();

        let p2 = JsonFilePersistence::new(&base, true, false);
        let reloaded = p2.get_user_data().await.unwrap();
        assert_eq!(reloaded.len(), 20);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn json_file_drop_chat_data_concurrent() {
        let dir = std::env::temp_dir().join("tg_stress_json_drop_chat");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let base = dir.join("drop_chat_data");

        let persistence = Arc::new(JsonFilePersistence::new(&base, true, true));

        for i in 0..40i64 {
            let mut data = JsonMap::new();
            data.insert("init".into(), Value::Number(i.into()));
            persistence.update_chat_data(i, &data).await.unwrap();
        }

        let barrier = Arc::new(Barrier::new(40));
        let mut handles = Vec::with_capacity(40);

        for i in 0..40i64 {
            let p = Arc::clone(&persistence);
            let b = Arc::clone(&barrier);
            handles.push(tokio::spawn(async move {
                b.wait().await;
                if i % 2 == 0 {
                    p.drop_chat_data(i).await.unwrap();
                } else {
                    let mut data = JsonMap::new();
                    data.insert("updated".into(), Value::Bool(true));
                    p.update_chat_data(i, &data).await.unwrap();
                }
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let chat_data = persistence.get_chat_data().await.unwrap();

        for i in (0..40i64).step_by(2) {
            assert!(!chat_data.contains_key(&i));
        }

        for i in (1..40i64).step_by(2) {
            let entry = chat_data.get(&i).unwrap();
            assert_eq!(entry.get("updated"), Some(&Value::Bool(true)));
        }

        // Flush and verify.
        persistence.flush().await.unwrap();

        let p2 = JsonFilePersistence::new(&base, true, false);
        let reloaded = p2.get_chat_data().await.unwrap();
        assert_eq!(reloaded.len(), 20);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
