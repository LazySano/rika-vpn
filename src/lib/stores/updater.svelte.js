import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { push } from "./notify.svelte.js";

export const updater = $state({
  checking: false,
  available: null,
  downloading: false,
  progress: 0,
  error: null,
});

export async function checkForUpdates(silent = false) {
  updater.checking = true;
  updater.error = null;
  try {
    const update = await check();
    if (update) {
      updater.available = {
        version: update.version,
        notes: update.body,
        date: update.date,
      };
      push("info", "يتوفر تحديث جديد", `النسخة ${update.version} جاهزة للتنزيل.`);
    } else {
      updater.available = null;
      if (!silent) push("success", "لا يوجد تحديث", "أنت على أحدث إصدار.");
    }
  } catch (e) {
    updater.error = String(e);
    if (!silent) push("error", "تعذّر التحقق من التحديثات", String(e));
  } finally {
    updater.checking = false;
  }
}

export async function installUpdate() {
  updater.downloading = true;
  updater.progress = 0;
  updater.error = null;
  try {
    const update = await check();
    if (!update) {
      updater.available = null;
      updater.downloading = false;
      return;
    }
    let total = 0;
    let downloaded = 0;
    await update.downloadAndInstall((event) => {
      if (event.event === "Started") {
        total = event.data.contentLength || 0;
      } else if (event.event === "Progress") {
        downloaded += event.data.chunkLength || 0;
        updater.progress = total ? Math.min(100, Math.round((downloaded / total) * 100)) : 0;
      } else if (event.event === "Finished") {
        updater.progress = 100;
      }
    });
    push("success", "تم تنزيل التحديث", "سيُعاد تشغيل التطبيق الآن.");
    await relaunch();
  } catch (e) {
    updater.error = String(e);
    push("error", "فشل تثبيت التحديث", String(e));
  } finally {
    updater.downloading = false;
  }
}
