<script>
  import Logo from "./Logo.svelte";
  import { auth, loginWithGoogle } from "$lib/stores/auth.svelte.js";
  import { ShieldCheck, Zap, Globe, LoaderCircle } from "@lucide/svelte";

  const features = [
    { icon: ShieldCheck, text: "اتصال مشفّر وآمن" },
    { icon: Zap, text: "سرعة عالية عبر WireGuard" },
    { icon: Globe, text: "سيرفرات حول العالم" },
  ];
</script>

<div
  class="relative flex h-screen w-screen items-center justify-center overflow-hidden bg-surface-2 text-ink"
>
  <div class="pointer-events-none absolute inset-0 opacity-80">
    <div class="absolute -top-24 -start-24 h-80 w-80 rounded-full bg-brand/25 blur-3xl"></div>
    <div class="absolute bottom-0 end-[20%] h-80 w-80 rounded-full bg-danger/10 blur-3xl"></div>
  </div>

  <div
    class="relative z-10 w-full max-w-md rounded-[28px] border border-line bg-surface p-8 text-center shadow-xl"
  >
    <div class="flex justify-center"><Logo size={46} /></div>

    <h1 class="mt-5 text-2xl font-extrabold text-ink">أهلًا بك في RikaVPN</h1>
    <p class="mt-2 text-sm text-muted">
      سجّل الدخول بحساب Google للبدء — بلا كلمات مرور ولا بيانات إضافية.
    </p>

    <button
      onclick={loginWithGoogle}
      disabled={auth.busy}
      class="mt-7 flex w-full items-center justify-center gap-3 rounded-2xl border border-line bg-surface px-4 py-3.5 text-base font-bold text-ink transition hover:bg-surface-2 disabled:opacity-70"
    >
      {#if auth.busy}
        <LoaderCircle size={20} class="animate-spin text-brand" /> جارٍ الدخول…
      {:else}
        <svg width="20" height="20" viewBox="0 0 48 48" aria-hidden="true">
          <path
            fill="#EA4335"
            d="M24 9.5c3.54 0 6.71 1.22 9.21 3.6l6.85-6.85C35.9 2.38 30.47 0 24 0 14.62 0 6.51 5.38 2.56 13.22l7.98 6.19C12.43 13.72 17.74 9.5 24 9.5z"
          />
          <path
            fill="#4285F4"
            d="M46.98 24.55c0-1.57-.15-3.09-.38-4.55H24v9.02h12.94c-.58 2.96-2.26 5.48-4.78 7.18l7.73 6c4.51-4.18 7.09-10.36 7.09-17.65z"
          />
          <path
            fill="#FBBC05"
            d="M10.53 28.59c-.48-1.45-.76-2.99-.76-4.59s.27-3.14.76-4.59l-7.98-6.19C.92 16.46 0 20.12 0 24s.92 7.54 2.56 10.78l7.97-6.19z"
          />
          <path
            fill="#34A853"
            d="M24 48c6.48 0 11.93-2.13 15.89-5.81l-7.73-6c-2.15 1.45-4.92 2.3-8.16 2.3-6.26 0-11.57-4.22-13.47-9.91l-7.98 6.19C6.51 42.62 14.62 48 24 48z"
          />
        </svg>
        المتابعة باستخدام Google
      {/if}
    </button>

    {#if auth.error}
      <p class="mt-3 rounded-xl bg-danger/10 px-3 py-2 text-xs leading-5 text-danger">
        {auth.error}
      </p>
    {/if}

    <div class="mt-7 grid grid-cols-3 gap-3 border-t border-line pt-6 text-center">
      {#each features as f}
        {@const Icon = f.icon}
        <div class="flex flex-col items-center gap-2">
          <span class="grid h-10 w-10 place-items-center rounded-full bg-brand-soft text-brand">
            <Icon size={18} />
          </span>
          <span class="text-[11px] leading-4 text-muted">{f.text}</span>
        </div>
      {/each}
    </div>
  </div>
</div>
