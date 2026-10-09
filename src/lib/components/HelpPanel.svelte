<script>
  import { X, FileKey, ShieldCheck, Wifi, Bell, Lock } from "@lucide/svelte";

  let { onClose } = $props();

  const steps = [
    {
      icon: FileKey,
      title: "استورد ملف تعريف",
      text: "من الإعدادات اختر ملف WireGuard ‎(.conf)‎ أو OpenVPN ‎(.ovpn)‎.",
    },
    {
      icon: ShieldCheck,
      title: "شغّل كمسؤول",
      text: "عند أول اتصال اضغط «إعادة التشغيل كمسؤول» لمنح صلاحية إنشاء واجهة الشبكة.",
    },
    {
      icon: Wifi,
      title: "اتصل",
      text: "من اللوحة اختر الدولة ثم اضغط زر «اتصال».",
    },
    {
      icon: Bell,
      title: "تابع الحالة",
      text: "الإشعارات تُخبرك بالنجاح أو الأخطاء، وصفحة تحليل الشبكة تقيس الجودة فعليًا.",
    },
    {
      icon: Lock,
      title: "مفتاح الحماية",
      text: "فعّله من الإعدادات لحجب الشبكة عند انقطاع النفق (يتطلب صلاحيات مدير).",
    },
  ];
</script>

<div
  class="absolute end-0 top-14 z-50 w-[380px] max-w-[92vw] overflow-hidden rounded-3xl border border-line bg-surface shadow-2xl"
>
  <div class="flex items-center justify-between border-b border-line px-4 py-3">
    <span class="font-bold text-ink">دليل الاستخدام السريع</span>
    <button
      onclick={onClose}
      class="rounded-lg p-2 text-muted transition hover:text-ink"
      aria-label="إغلاق"
    >
      <X size={16} />
    </button>
  </div>

  <div class="max-h-[60vh] space-y-4 overflow-y-auto p-4">
    {#each steps as s, i}
      {@const Icon = s.icon}
      <div class="flex gap-3">
        <span
          class="grid h-8 w-8 shrink-0 place-items-center rounded-full bg-brand-soft text-brand"
        >
          <Icon size={16} />
        </span>
        <div>
          <div class="text-sm font-bold text-ink">{i + 1}. {s.title}</div>
          <p class="text-xs leading-5 text-muted">{s.text}</p>
        </div>
      </div>
    {/each}

    <p class="rounded-xl bg-surface-2 p-3 text-xs leading-5 text-muted">
      للاتصال بسيرفرات الدول يجب استيراد ملف اتصال حقيقي تملكه. بدون ملف، تبقى الواجهة
      ومراقبة الشبكة واختبار السرعة تعمل ببيانات فعلية.
    </p>
  </div>
</div>
