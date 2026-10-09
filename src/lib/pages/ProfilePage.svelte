<script>
  import { Mail, Crown, Calendar, Laptop, AtSign } from "@lucide/svelte";
  import { auth, userInitials } from "$lib/stores/auth.svelte.js";

  function providerLabel(p) {
    return p === "google" ? "حساب Google" : "البريد الإلكتروني";
  }

  function joined(ts) {
    if (!ts) return "غير معروف";
    return new Date(ts).toLocaleDateString("ar-EG", {
      year: "numeric",
      month: "long",
      day: "numeric",
    });
  }

  const rows = $derived([
    { icon: Mail, label: "البريد الإلكتروني", value: auth.session?.email ?? "—" },
    { icon: AtSign, label: "طريقة الدخول", value: providerLabel(auth.session?.provider) },
    { icon: Crown, label: "الخطة", value: "مجاني بالكامل" },
    { icon: Calendar, label: "عضو منذ", value: joined(auth.session?.createdAt) },
    { icon: Laptop, label: "الأجهزة", value: "ويندوز — جهاز واحد" },
  ]);
</script>

<div class="mx-auto max-w-2xl">
  <div class="rounded-[28px] border border-line bg-surface p-8 shadow-sm">
    <div class="flex flex-col items-center gap-2">
      <div
        class="grid h-24 w-24 place-items-center rounded-full bg-gradient-to-br from-[#8b7bff] to-brand text-3xl font-bold text-white shadow-lg"
      >
        {userInitials()}
      </div>
      <div class="mt-2 text-2xl font-extrabold text-ink">{auth.session?.name ?? "مستخدم"}</div>
      <span dir="ltr" class="text-muted">{auth.session?.email ?? ""}</span>
    </div>

    <div class="mt-8 divide-y divide-line">
      {#each rows as r}
        {@const Icon = r.icon}
        <div class="flex items-center justify-between py-4">
          <span class="flex items-center gap-3 text-muted"><Icon size={18} /> {r.label}</span>
          <span dir="auto" class="font-semibold text-ink">{r.value}</span>
        </div>
      {/each}
    </div>
  </div>
</div>
