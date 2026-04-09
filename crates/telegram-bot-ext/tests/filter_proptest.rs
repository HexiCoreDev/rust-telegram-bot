//! Property-based tests for filter composition operators.
//!
//! Verifies that the bitwise combinators on [`F`] satisfy the expected
//! algebraic laws: conjunction, disjunction, negation, exclusive-or,
//! idempotence, double negation, and De Morgan's laws.

use proptest::prelude::*;

use rust_tg_bot_ext::filters::base::{Filter, FilterResult, FnFilter, F};

type Update = rust_tg_bot_ext::filters::base::Update;

// ---------------------------------------------------------------------------
// Strategy: generate a minimal Update from JSON
// ---------------------------------------------------------------------------

/// Produce a valid [`Update`] that is either a bare `{"update_id": N}` or a
/// message-carrying update.  The choice is driven by proptest so that filters
/// see both "empty" and "message" updates across many iterations.
fn update_strategy() -> impl Strategy<Value = Update> {
    prop_oneof![
        // Bare update (no effective message)
        (1..=1000i64).prop_map(|id| {
            serde_json::from_value(serde_json::json!({ "update_id": id })).unwrap()
        }),
        // Message update with text
        (1..=1000i64).prop_map(|id| {
            serde_json::from_value(serde_json::json!({
                "update_id": id,
                "message": {
                    "message_id": 1,
                    "date": 0,
                    "chat": { "id": 1, "type": "private" },
                    "text": "hello"
                }
            }))
            .unwrap()
        }),
    ]
}

// ---------------------------------------------------------------------------
// Strategy: generate a simple filter (always-true, always-false, or random)
// ---------------------------------------------------------------------------

/// Represents the three kinds of test filters we compose.
#[derive(Debug, Clone, Copy)]
enum FilterKind {
    AlwaysTrue,
    AlwaysFalse,
    /// Matches only when the update carries a message field.
    HasMessage,
}

fn filter_kind_strategy() -> impl Strategy<Value = FilterKind> {
    prop_oneof![
        Just(FilterKind::AlwaysTrue),
        Just(FilterKind::AlwaysFalse),
        Just(FilterKind::HasMessage),
    ]
}

/// Evaluate a [`FilterKind`] against an [`Update`] -- the "expected" side of
/// our property assertions.  Uses the typed `message` field directly to avoid
/// repeated serialization.
fn eval(kind: FilterKind, update: &Update) -> bool {
    match kind {
        FilterKind::AlwaysTrue => true,
        FilterKind::AlwaysFalse => false,
        FilterKind::HasMessage => update.message().is_some(),
    }
}

/// Build an [`F`] wrapper from a [`FilterKind`].
fn make_filter(kind: FilterKind) -> F {
    match kind {
        FilterKind::AlwaysTrue => F::new(FnFilter::new("always_true", |_| true)),
        FilterKind::AlwaysFalse => F::new(FnFilter::new("always_false", |_| false)),
        FilterKind::HasMessage => F::new(FnFilter::new("has_message", |u: &Update| {
            u.message().is_some()
        })),
    }
}

fn is_match(result: &FilterResult) -> bool {
    result.is_match()
}

// ---------------------------------------------------------------------------
// Property tests
// ---------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    // (A & B).check(u) == A.check(u) && B.check(u)
    #[test]
    fn and_is_conjunction(
        a_kind in filter_kind_strategy(),
        b_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let a = make_filter(a_kind);
        let b = make_filter(b_kind);
        let combined = a.clone() & b.clone();

        let expected = eval(a_kind, &update) && eval(b_kind, &update);
        let actual = is_match(&combined.check_update(&update));
        prop_assert_eq!(actual, expected, "AND conjunction failed");
    }

    // (A | B).check(u) == A.check(u) || B.check(u)
    #[test]
    fn or_is_disjunction(
        a_kind in filter_kind_strategy(),
        b_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let a = make_filter(a_kind);
        let b = make_filter(b_kind);
        let combined = a.clone() | b.clone();

        let expected = eval(a_kind, &update) || eval(b_kind, &update);
        let actual = is_match(&combined.check_update(&update));
        prop_assert_eq!(actual, expected, "OR disjunction failed");
    }

    // (!A).check(u) == !A.check(u)
    #[test]
    fn not_is_negation(
        a_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let a = make_filter(a_kind);
        let negated = !a.clone();

        let expected = !eval(a_kind, &update);
        let actual = is_match(&negated.check_update(&update));
        prop_assert_eq!(actual, expected, "NOT negation failed");
    }

    // (A ^ B).check(u) == (A.check(u) ^ B.check(u))
    #[test]
    fn xor_is_exclusive_or(
        a_kind in filter_kind_strategy(),
        b_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let a = make_filter(a_kind);
        let b = make_filter(b_kind);
        let combined = a.clone() ^ b.clone();

        let expected = eval(a_kind, &update) ^ eval(b_kind, &update);
        let actual = is_match(&combined.check_update(&update));
        prop_assert_eq!(actual, expected, "XOR exclusive-or failed");
    }

    // (A & A).check(u) == A.check(u) -- AND idempotent
    #[test]
    fn and_idempotent(
        a_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let a1 = make_filter(a_kind);
        let a2 = make_filter(a_kind);
        let combined = a1 & a2;

        let expected = eval(a_kind, &update);
        let actual = is_match(&combined.check_update(&update));
        prop_assert_eq!(actual, expected, "AND idempotence failed");
    }

    // (A | A).check(u) == A.check(u) -- OR idempotent
    #[test]
    fn or_idempotent(
        a_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let a1 = make_filter(a_kind);
        let a2 = make_filter(a_kind);
        let combined = a1 | a2;

        let expected = eval(a_kind, &update);
        let actual = is_match(&combined.check_update(&update));
        prop_assert_eq!(actual, expected, "OR idempotence failed");
    }

    // !!A == A -- double negation
    #[test]
    fn double_negation(
        a_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let a = make_filter(a_kind);
        let double_neg = !(!a.clone());

        let expected = eval(a_kind, &update);
        let actual = is_match(&double_neg.check_update(&update));
        prop_assert_eq!(actual, expected, "Double negation failed");
    }

    // De Morgan's: !(A & B) == (!A | !B)
    #[test]
    fn de_morgan_and(
        a_kind in filter_kind_strategy(),
        b_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let lhs = !(make_filter(a_kind) & make_filter(b_kind));
        let rhs = !make_filter(a_kind) | !make_filter(b_kind);

        let lhs_result = is_match(&lhs.check_update(&update));
        let rhs_result = is_match(&rhs.check_update(&update));
        prop_assert_eq!(lhs_result, rhs_result, "De Morgan AND failed");
    }

    // De Morgan's: !(A | B) == (!A & !B)
    #[test]
    fn de_morgan_or(
        a_kind in filter_kind_strategy(),
        b_kind in filter_kind_strategy(),
        update in update_strategy(),
    ) {
        let lhs = !(make_filter(a_kind) | make_filter(b_kind));
        let rhs = !make_filter(a_kind) & !make_filter(b_kind);

        let lhs_result = is_match(&lhs.check_update(&update));
        let rhs_result = is_match(&rhs.check_update(&update));
        prop_assert_eq!(lhs_result, rhs_result, "De Morgan OR failed");
    }
}
