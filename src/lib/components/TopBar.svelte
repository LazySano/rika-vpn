<script>
  import { Sun, Moon, Bell, CircleHelp } from "@lucide/svelte";
  import Logo from "./Logo.svelte";
  import NotificationsPanel from "./NotificationsPanel.svelte";
  import HelpPanel from "./HelpPanel.svelte";
  import { app, toggleTheme } from "$lib/stores/app.svelte.js";
  import { notify, markAllRead } from "$lib/stores/notify.svelte.js";

  let showNotifs = $state(false);
  let showHelp = $state(false);

  function toggleNotifs() {
    showHelp = false;
    showNotifs = !showNotifs;
    if (showNotifs) markAllRead();
  }

  function toggleHelp() {
    showNotifs = false;
    showHelp = !showHelp;
  }

  function closeAll() {
    showNotifs = false;
    showHelp = false;
  }
</script>

<header class="relative flex items-center justify-between px-7 py-4">
  <Logo />

  <div class="flex items-center gap-3">
    <div class="flex items-center gap-1 rounded-full bg-surface-2 p-1">
      <button
        onclick={toggleTheme}
        aria-label="الوضع الفاتح"
        class="grid h-8 w-8 place-items-center rounded-full transition {app.theme === 'light'
          ? 'bg-surface text-brand shadow'
          : 'text-muted hover:text-ink'}"
      >
        <Sun size={16} />
      </button>
      <button
        onclick={toggleTheme}
        aria-label="الوضع المظلم"
        class="grid h-8 w-8 place-items-center rounded-full transition {app.theme === 'dark'
          ? 'bg-surface text-brand shadow'
          : 'text-muted hover:text-ink'}"
      >
        <Moon size={16} />
      </button>
    </div>

    <button
      onclick={toggleHelp}
      aria-label="المساعدة"
      class="grid h-10 w-10 place-items-center rounded-full bg-surface-2 text-muted transition hover:text-ink"
    >
      <CircleHelp size={18} />
    </button>

    <button
      onclick={toggleNotifs}
      aria-label="الإشعارات"
      class="relative grid h-10 w-10 place-items-center rounded-full bg-surface-2 text-muted transition hover:text-ink"
    >
      <Bell size={18} />
      {#if notify.unread > 0}
        <span
          class="absolute -top-1 -end-1 grid h-4 min-w-4 place-items-center rounded-full bg-danger px-1 text-[9px] font-bold text-white"
        >
          {notify.unread}
        </span>
      {/if}
    </button>
  </div>

  {#if showNotifs || showHelp}
    <button
      class="fixed inset-0 z-40 cursor-default"
      onclick={closeAll}
      aria-label="إغلاق اللوحة"
    ></button>
  {/if}

  {#if showNotifs}
    <NotificationsPanel onClose={closeAll} />
  {/if}
  {#if showHelp}
    <HelpPanel onClose={closeAll} />
  {/if}
</header>
