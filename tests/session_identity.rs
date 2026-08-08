#![forbid(unsafe_code)]

use skill_evidence::{ErrorClass, resolve_top_level_session_id};

#[test]
fn top_level_session_resolution_is_host_neutral_and_fails_closed_on_conflict() {
    assert_eq!(
        resolve_top_level_session_id(
            Some("explicit"),
            Some("claude-conflict"),
            Some("codex-conflict")
        )
        .expect("explicit identity wins"),
        "explicit"
    );
    assert_eq!(
        resolve_top_level_session_id(Some(""), Some("claude"), None)
            .expect("explicit empty identity preserves unavailable behavior"),
        "unavailable"
    );
    assert_eq!(
        resolve_top_level_session_id(None, Some("claude"), None).expect("Claude identity resolves"),
        "claude"
    );
    assert_eq!(
        resolve_top_level_session_id(None, None, Some("codex")).expect("Codex identity resolves"),
        "codex"
    );
    assert_eq!(
        resolve_top_level_session_id(None, None, None).expect("absent identities resolve"),
        "unavailable"
    );
    assert_eq!(
        resolve_top_level_session_id(None, Some("same"), Some("same"))
            .expect("matching host identities resolve"),
        "same"
    );

    let conflict = resolve_top_level_session_id(None, Some("claude"), Some("codex"))
        .expect_err("conflicting identities must refuse");
    assert_eq!(conflict.class(), ErrorClass::Refusal);
    let message = conflict.to_string();
    assert!(message.contains("Conflicting top-level-session identities"));
    assert!(message.contains("CLAUDE_CODE_SESSION_ID=claude"));
    assert!(message.contains("CODEX_THREAD_ID=codex"));
}
