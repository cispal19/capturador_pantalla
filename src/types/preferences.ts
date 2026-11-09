export interface AppPreferences {
  capture_shortcut: string;
  default_save_directory?: string | null;
  default_image_format: "png" | "jpeg";
  autostart_enabled: boolean;
  show_notifications: boolean;
  copy_to_clipboard_on_capture: boolean;
}

export const DEFAULT_PREFERENCES: AppPreferences = {
  capture_shortcut: "CommandOrControl+Shift+X",
  default_save_directory: null,
  default_image_format: "png",
  autostart_enabled: false,
  show_notifications: true,
  copy_to_clipboard_on_capture: true,
};
