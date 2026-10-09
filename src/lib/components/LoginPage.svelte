<script>
  import { Eye, EyeOff, Mail, Lock, User } from "@lucide/svelte";
  import Logo from "./Logo.svelte";
  import { auth, login, register, loginWithGoogle } from "$lib/stores/auth.svelte.js";

  let mode = $state("login");
  let name = $state("");
  let email = $state("");
  let password = $state("");
  let show = $state(false);

  function switchMode(m) {
    mode = m;
    auth.error = null;
  }

  async function submit(event) {
    event.preventDefault();
    if (mode === "login") await login(email, password);
    else await register(name, email, password);
  }
</script>

<div
  class="relative flex h-screen w-screen items-center justify-center overflow-hidden bg-surface-2 text-ink"
>
  <div class="pointer-events-none absolute inset-0 opacity-80">
    <div class="absolute -top-24 -start-24 h-80 w-80 rounded-full bg-brand/25 blur-3xl"></div>
    <div class="absolute bottom-0 end-[20%] h-80 w-80 rounded-full bg-danger/10 blur-3xl"></div>
  </div>

  <div class="relative z-10 w-full max-w-md rounded-[28px] border border-line bg-surface p-8 shadow-xl">
    <div class="flex justify-center"><Logo size={40} /></div>
    <p class="mt-3 text-center text-sm text-muted">اتصال آمن وسريع حول العالم</p>

    <div class="mt-6 grid grid-cols-2 gap-1 rounded-2xl bg-surface-2 p-1">
      <button
        onclick={() => switchMode("login")}
        class="rounded-xl py-2 text-sm font-bold transition {mode === 'login' ? 'bg-brand text-white' : 'text-muted'}"
      >
        تسجيل الدخول
      </button>
      <button
        onclick={() => switchMode("register")}
        class="rounded-xl py-2 text-sm font-bold transition {mode === 'register' ? 'bg-brand text-white' : 'text-muted'}"
      >
        إنشاء حساب
      </button>
    </div>

    <form onsubmit={submit} class="mt-6 space-y-3">
      {#if mode === "register"}
        <label class="flex items-center gap-3 rounded-2xl bg-surface-2 px-4 py-3">
          <User size={18} class="shrink-0 text-muted" />
          <input
            bind:value={name}
            placeholder="الاسم الكامل"
            class="w-full bg-transparent text-sm outline-none"
          />
        </label>
      {/if}
      <label class="flex items-center gap-3 rounded-2xl bg-surface-2 px-4 py-3">
        <Mail size={18} class="shrink-0 text-muted" />
        <input
          bind:value={email}
          type="email"
          placeholder="البريد الإلكتروني"
          dir="ltr"
          class="w-full bg-transparent text-sm outline-none"
        />
      </label>
      <label class="flex items-center gap-3 rounded-2xl bg-surface-2 px-4 py-3">
        <Lock size={18} class="shrink-0 text-muted" />
        <input
          bind:value={password}
          type={show ? "text" : "password"}
          placeholder="كلمة المرور"
          dir="ltr"
          class="w-full bg-transparent text-sm outline-none"
        />
        <button
          type="button"
          onclick={() => (show = !show)}
          aria-label="إظهار كلمة المرور"
          class="text-muted"
        >
          {#if show}<EyeOff size={16} />{:else}<Eye size={16} />{/if}
        </button>
      </label>

      {#if auth.error}
        <p class="rounded-xl bg-danger/10 px-3 py-2 text-xs text-danger">{auth.error}</p>
      {/if}

      <button
        type="submit"
        disabled={auth.busy}
        class="w-full rounded-2xl bg-brand py-3 font-bold text-white transition hover:bg-brand-hover disabled:opacity-70"
      >
        {mode === "login" ? "دخول" : "إنشاء الحساب"}
      </button>
    </form>

    <div class="my-4 flex items-center gap-3 text-xs text-muted">
      <span class="h-px flex-1 bg-line"></span> أو <span class="h-px flex-1 bg-line"></span>
    </div>

    <button
      onclick={loginWithGoogle}
      class="flex w-full items-center justify-center gap-3 rounded-2xl border border-line py-3 text-sm font-semibold text-ink transition hover:bg-surface-2"
    >
      <span class="text-base font-black text-[#4285F4]">G</span> المتابعة باستخدام Google
    </button>
  </div>
</div>
