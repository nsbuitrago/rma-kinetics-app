import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

export interface UpdateState {
  checking: boolean;
  available: boolean;
  downloading: boolean;
  progress: number;
  update: Update | null;
  error: string | null;
}

let updateDialogCallback: ((state: UpdateState) => void) | null = null;

export function onUpdateStateChange(callback: (state: UpdateState) => void) {
  updateDialogCallback = callback;
}

const state: UpdateState = {
  checking: false,
  available: false,
  downloading: false,
  progress: 0,
  update: null,
  error: null,
};

function notifyStateChange() {
  if (updateDialogCallback) {
    updateDialogCallback({ ...state });
  }
}

export async function checkForUpdate(): Promise<Update | null> {
  try {
    state.checking = true;
    state.error = null;
    notifyStateChange();

    const update = await check();

    if (update) {
      state.available = true;
      state.update = update;
      console.log(
        `Update available: ${update.version} from ${update.date} - ${update.body}`,
      );
    } else {
      state.available = false;
      state.update = null;
      console.log("No updates available");
    }

    state.checking = false;
    notifyStateChange();

    return update;
  } catch (error) {
    console.error("Failed to check for updates:", error);
    state.checking = false;
    state.error = error instanceof Error ? error.message : String(error);
    notifyStateChange();
    return null;
  }
}

export async function downloadAndInstall(
  update: Update,
  onProgress?: (downloaded: number, total: number | undefined) => void,
): Promise<void> {
  try {
    state.downloading = true;
    state.progress = 0;
    state.error = null;
    notifyStateChange();

    let downloaded = 0;
    let contentLength: number | undefined;

    await update.downloadAndInstall((event) => {
      switch (event.event) {
        case "Started":
          contentLength = event.data.contentLength ?? undefined;
          console.log(`Started downloading ${contentLength} bytes`);
          break;
        case "Progress":
          downloaded += event.data.chunkLength;
          state.progress = contentLength
            ? Math.round((downloaded / contentLength) * 100)
            : 0;
          notifyStateChange();
          if (onProgress) {
            onProgress(downloaded, contentLength);
          }
          console.log(`Downloaded ${downloaded} from ${contentLength}`);
          break;
        case "Finished":
          state.progress = 100;
          console.log("Download finished");
          notifyStateChange();
          break;
      }
    });

    console.log("Update installed successfully");
    state.downloading = false;
    notifyStateChange();

    // Relaunch the app
    await relaunch();
  } catch (error) {
    console.error("Failed to download and install update:", error);
    state.downloading = false;
    state.error = error instanceof Error ? error.message : String(error);
    notifyStateChange();
    throw error;
  }
}

export function dismissUpdate() {
  state.available = false;
  state.update = null;
  notifyStateChange();
}
