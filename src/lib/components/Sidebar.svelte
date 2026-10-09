<script>
  import {
    LayoutDashboard,
    User,
    Gauge,
    RadioTower,
    Info,
    Settings,
    LogOut,
    BadgeCheck,
    ChevronDown,
  } from "@lucide/svelte";
  import { app, setPage } from "$lib/stores/app.svelte.js";
  import { auth, logout, userInitials } from "$lib/stores/auth.svelte.js";
  import { push } from "$lib/stores/notify.svelte.js";

  const mainItems = [
    { id: "dashboard", label: "لوحتي", icon: LayoutDashboard },
    { id: "profile", label: "ملفي الشخصي", icon: User },
    { id: "speed", label: "اختبار السرعة", icon: Gauge },
    { id: "network", label: "تحليل الشبكة", icon: RadioTower },
  ];

  const footItems = [
    { id: "about", label: "حول RikaVPN", icon: Info },
    { id: "settings", label: "الإعدادات", icon: Settings },
  ];

  function handleLogout() {
    const name = auth.session?.name ?? "";
    logout();
    push("info", "تم تسجيل الخروج", `إلى اللقاء ${name}`);
  }
</script>

<aside class="flex w-[258px] shrink-0 flex-col border-e border-line bg-surface">
  <div class="relative">
    <div
      class="h-28 w-full rounded-b-[30px] bg-gradient-to-br from-[#8b7bff] via-brand to-[#5a49d6]"
    ></div>
    <div class="absolute inset-x-0 -bottom-9 flex justify-center">
      <div class="relative">
        <div
          class="grid h-[74px] w-[74px] place-items-center rounded-full border-4 border-surface bg-gradient-to-br from-[#8b7bff] to-brand text-2xl font-bold text-white shadow-lg"
        >
          {userInitials()}
        </div>
        <span
          class="absolute -bottom-0.5 -end-0.5 grid h-6 w-6 place-items-center rounded-full bg-brand text-white ring-2 ring-surface"
        >
          <BadgeCheck size={14} />
        </span>
      </div>
    </div>
  </div>

  <div class="mt-12 flex flex-col items-center gap-0.5 px-4">
    <button class="flex items-center gap-1 text-lg font-bold text-ink">
      {auth.session?.name ?? "مستخدم"}
      <ChevronDown size={16} />
    </button>
    <span dir="ltr" class="text-xs text-muted">{auth.session?.email ?? ""}</span>
  </div>

  <nav class="mt-6 flex flex-1 flex-col gap-1 overflow-y-auto px-3">
    {#each mainItems as it (it.id)}
      {@const Icon = it.icon}
      <button
        onclick={() => setPage(it.id)}
        class="flex items-center gap-3 rounded-2xl px-4 py-3 text-sm font-semibold transition-colors {app.page ===
        it.id
          ? 'bg-brand-soft text-brand'
          : 'text-muted hover:bg-surface-2 hover:text-ink'}"
      >
        <Icon size={20} />
        <span class="flex-1 text-start">{it.label}</span>
      </button>
    {/each}

    <div class="my-3 h-px bg-line"></div>

    {#each footItems as it (it.id)}
      {@const Icon = it.icon}
      <button
        onclick={() => setPage(it.id)}
        class="flex items-center gap-3 rounded-2xl px-4 py-3 text-sm font-semibold transition-colors {app.page ===
        it.id
          ? 'bg-brand-soft text-brand'
          : 'text-muted hover:bg-surface-2 hover:text-ink'}"
      >
        <Icon size={20} />
        <span class="flex-1 text-start">{it.label}</span>
      </button>
    {/each}
  </nav>

  <div class="p-3">
    <button
      onclick={handleLogout}
      class="flex w-full items-center gap-3 rounded-2xl px-4 py-3 text-sm font-semibold text-danger transition-colors hover:bg-danger/10"
    >
      <LogOut size={20} />
      <span class="flex-1 text-start">تسجيل الخروج</span>
    </button>
  </div>
</aside>
