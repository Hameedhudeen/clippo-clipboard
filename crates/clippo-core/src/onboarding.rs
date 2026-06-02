use serde::{Deserialize, Serialize};

use crate::ClippoSettings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OnboardingStepId {
    ClipboardAccess,
    PasteAutomation,
    GlobalShortcut,
    PrivacyDefaults,
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingStep {
    pub id: OnboardingStepId,
    pub title_key: String,
    pub body_key: String,
    pub action_label_key: Option<String>,
    pub required: bool,
    pub completed: bool,
    pub shortcut_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OnboardingProgress {
    pub completed_steps: Vec<OnboardingStepId>,
}

impl OnboardingProgress {
    #[must_use]
    pub fn is_completed(&self, step_id: OnboardingStepId) -> bool {
        self.completed_steps.contains(&step_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OnboardingViewModel {
    pub steps: Vec<OnboardingStep>,
    pub current_step: Option<OnboardingStepId>,
    pub complete: bool,
}

impl OnboardingViewModel {
    #[must_use]
    pub fn from_progress(settings: &ClippoSettings, progress: &OnboardingProgress) -> Self {
        let steps = vec![
            OnboardingStep {
                id: OnboardingStepId::ClipboardAccess,
                title_key: "onboarding.clipboard.title".to_string(),
                body_key: "onboarding.clipboard.body".to_string(),
                action_label_key: Some("onboarding.clipboard.action".to_string()),
                required: true,
                completed: progress.is_completed(OnboardingStepId::ClipboardAccess),
                shortcut_hint: None,
            },
            OnboardingStep {
                id: OnboardingStepId::PasteAutomation,
                title_key: "onboarding.paste.title".to_string(),
                body_key: "onboarding.paste.body".to_string(),
                action_label_key: Some("onboarding.paste.action".to_string()),
                required: false,
                completed: progress.is_completed(OnboardingStepId::PasteAutomation)
                    || !settings.paste_automatically,
                shortcut_hint: None,
            },
            OnboardingStep {
                id: OnboardingStepId::GlobalShortcut,
                title_key: "onboarding.shortcut.title".to_string(),
                body_key: "onboarding.shortcut.body".to_string(),
                action_label_key: Some("onboarding.shortcut.action".to_string()),
                required: true,
                completed: progress.is_completed(OnboardingStepId::GlobalShortcut),
                shortcut_hint: Some(settings.shortcuts.open_history.clone()),
            },
            OnboardingStep {
                id: OnboardingStepId::PrivacyDefaults,
                title_key: "onboarding.privacy.title".to_string(),
                body_key: "onboarding.privacy.body".to_string(),
                action_label_key: Some("onboarding.privacy.action".to_string()),
                required: true,
                completed: progress.is_completed(OnboardingStepId::PrivacyDefaults),
                shortcut_hint: None,
            },
            OnboardingStep {
                id: OnboardingStepId::Complete,
                title_key: "onboarding.complete.title".to_string(),
                body_key: "onboarding.complete.body".to_string(),
                action_label_key: Some("onboarding.complete.action".to_string()),
                required: false,
                completed: true,
                shortcut_hint: None,
            },
        ];

        let current_step = steps
            .iter()
            .find(|step| step.required && !step.completed)
            .or_else(|| steps.iter().find(|step| !step.completed))
            .map(|step| step.id);
        let complete = current_step.is_none();

        Self {
            steps,
            current_step: if complete {
                Some(OnboardingStepId::Complete)
            } else {
                current_step
            },
            complete,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_run_onboarding_starts_with_clipboard_access() {
        let view_model = OnboardingViewModel::from_progress(
            &ClippoSettings::default(),
            &OnboardingProgress::default(),
        );

        assert_eq!(
            view_model.current_step,
            Some(OnboardingStepId::ClipboardAccess)
        );
        assert!(!view_model.complete);
        assert!(view_model
            .steps
            .iter()
            .any(|step| step.id == OnboardingStepId::GlobalShortcut
                && step.shortcut_hint == Some("Shift+Meta+C".to_string())));
    }

    #[test]
    fn onboarding_can_skip_paste_permission_when_auto_paste_is_disabled() {
        let settings = ClippoSettings {
            paste_automatically: false,
            ..ClippoSettings::default()
        };
        let view_model = OnboardingViewModel::from_progress(
            &settings,
            &OnboardingProgress {
                completed_steps: vec![
                    OnboardingStepId::ClipboardAccess,
                    OnboardingStepId::GlobalShortcut,
                    OnboardingStepId::PrivacyDefaults,
                ],
            },
        );

        assert!(view_model.complete);
        assert_eq!(view_model.current_step, Some(OnboardingStepId::Complete));
    }

    #[test]
    fn onboarding_requires_privacy_review_before_completion() {
        let view_model = OnboardingViewModel::from_progress(
            &ClippoSettings::default(),
            &OnboardingProgress {
                completed_steps: vec![
                    OnboardingStepId::ClipboardAccess,
                    OnboardingStepId::PasteAutomation,
                    OnboardingStepId::GlobalShortcut,
                ],
            },
        );

        assert_eq!(
            view_model.current_step,
            Some(OnboardingStepId::PrivacyDefaults)
        );
        assert!(!view_model.complete);
    }
}
