use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppLifecycleEvent {
    FirstLaunch,
    Launch,
    SecondLaunch,
    Sleep,
    Wake,
    MenuOrTrayQuitRequested,
    ShutdownRequested,
    CrashRecovered,
    UpdateRestartRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleStatus {
    Starting,
    Running,
    Suspended,
    ShuttingDown,
    RestartingForUpdate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct AppLifecycleState {
    pub status: LifecycleStatus,
    pub first_run_onboarding_required: bool,
    pub recovered_from_crash: bool,
    pub should_focus_existing_instance: bool,
    pub clipboard_monitoring_active: bool,
    pub should_save_before_exit: bool,
    pub should_relaunch_after_update: bool,
    pub shutdown_requested_from_menu_or_tray: bool,
}

impl Default for AppLifecycleState {
    fn default() -> Self {
        Self {
            status: LifecycleStatus::Starting,
            first_run_onboarding_required: false,
            recovered_from_crash: false,
            should_focus_existing_instance: false,
            clipboard_monitoring_active: false,
            should_save_before_exit: false,
            should_relaunch_after_update: false,
            shutdown_requested_from_menu_or_tray: false,
        }
    }
}

impl AppLifecycleState {
    #[must_use]
    pub fn apply(mut self, event: AppLifecycleEvent) -> Self {
        match event {
            AppLifecycleEvent::FirstLaunch => {
                self.status = LifecycleStatus::Running;
                self.first_run_onboarding_required = true;
                self.clipboard_monitoring_active = true;
            }
            AppLifecycleEvent::Launch => {
                self.status = LifecycleStatus::Running;
                self.should_focus_existing_instance = false;
                self.clipboard_monitoring_active = true;
            }
            AppLifecycleEvent::SecondLaunch => {
                self.should_focus_existing_instance = true;
            }
            AppLifecycleEvent::Sleep => {
                self.status = LifecycleStatus::Suspended;
                self.clipboard_monitoring_active = false;
            }
            AppLifecycleEvent::Wake => {
                self.status = LifecycleStatus::Running;
                self.clipboard_monitoring_active = true;
            }
            AppLifecycleEvent::MenuOrTrayQuitRequested => {
                self.status = LifecycleStatus::ShuttingDown;
                self.clipboard_monitoring_active = false;
                self.should_save_before_exit = true;
                self.shutdown_requested_from_menu_or_tray = true;
            }
            AppLifecycleEvent::ShutdownRequested => {
                self.status = LifecycleStatus::ShuttingDown;
                self.clipboard_monitoring_active = false;
                self.should_save_before_exit = true;
            }
            AppLifecycleEvent::CrashRecovered => {
                self.status = LifecycleStatus::Running;
                self.recovered_from_crash = true;
                self.clipboard_monitoring_active = true;
                self.should_save_before_exit = false;
            }
            AppLifecycleEvent::UpdateRestartRequested => {
                self.status = LifecycleStatus::RestartingForUpdate;
                self.clipboard_monitoring_active = false;
                self.should_save_before_exit = true;
                self.should_relaunch_after_update = true;
            }
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_launch_requires_onboarding() {
        let state = AppLifecycleState::default().apply(AppLifecycleEvent::FirstLaunch);

        assert_eq!(state.status, LifecycleStatus::Running);
        assert!(state.first_run_onboarding_required);
        assert!(state.clipboard_monitoring_active);
    }

    #[test]
    fn second_launch_focuses_existing_instance() {
        let state = AppLifecycleState::default()
            .apply(AppLifecycleEvent::Launch)
            .apply(AppLifecycleEvent::SecondLaunch);

        assert!(state.should_focus_existing_instance);
    }

    #[test]
    fn wake_resumes_running_state() {
        let state = AppLifecycleState::default()
            .apply(AppLifecycleEvent::Sleep)
            .apply(AppLifecycleEvent::Wake);

        assert_eq!(state.status, LifecycleStatus::Running);
        assert!(state.clipboard_monitoring_active);
    }

    #[test]
    fn shutdown_moves_to_shutting_down_state() {
        let state = AppLifecycleState::default().apply(AppLifecycleEvent::ShutdownRequested);

        assert_eq!(state.status, LifecycleStatus::ShuttingDown);
        assert!(state.should_save_before_exit);
        assert!(!state.clipboard_monitoring_active);
    }

    #[test]
    fn menu_or_tray_quit_requests_clean_shutdown() {
        let state = AppLifecycleState::default()
            .apply(AppLifecycleEvent::Launch)
            .apply(AppLifecycleEvent::MenuOrTrayQuitRequested);

        assert_eq!(state.status, LifecycleStatus::ShuttingDown);
        assert!(state.should_save_before_exit);
        assert!(state.shutdown_requested_from_menu_or_tray);
        assert!(!state.clipboard_monitoring_active);
    }

    #[test]
    fn crash_recovery_records_recovered_state() {
        let state = AppLifecycleState::default().apply(AppLifecycleEvent::CrashRecovered);

        assert_eq!(state.status, LifecycleStatus::Running);
        assert!(state.recovered_from_crash);
        assert!(state.clipboard_monitoring_active);
        assert!(!state.should_save_before_exit);
    }

    #[test]
    fn update_restart_moves_to_restart_state() {
        let state = AppLifecycleState::default().apply(AppLifecycleEvent::UpdateRestartRequested);

        assert_eq!(state.status, LifecycleStatus::RestartingForUpdate);
        assert!(state.should_save_before_exit);
        assert!(state.should_relaunch_after_update);
        assert!(!state.clipboard_monitoring_active);
    }
}
