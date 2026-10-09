export const t = {
  app: "RikaVPN",
  tagline: "اتصال آمن وسريع حول العالم",

  nav: {
    dashboard: "لوحتي",
    profile: "ملفي الشخصي",
    pro: "الترقية إلى PRO",
    speed: "اختبار السرعة",
    network: "تحليل الشبكة",
    about: "حول RikaVPN",
    settings: "الإعدادات",
    logout: "تسجيل الخروج",
  },

  server: {
    change: "تغيير السيرفر",
    searching: "أفضل سيرفر",
  },

  conn: {
    connected: "متصل",
    disconnected: "غير متصل",
    connecting: "جارٍ الاتصال…",
    connect: "اتصال",
    disconnect: "قطع الاتصال",
    ip: "عنوان IP",
    basic: "أساسي",
    download: "تنزيل",
    upload: "رفع",
    unit: "م.ب/ث",
  },

  speed: {
    title: "أعلى سرعة",
    unit: "ميغابت/ث",
    start: "ابدأ الاختبار",
    ping: "زمن الاستجابة",
    download: "سرعة التنزيل",
    upload: "سرعة الرفع",
    ms: "ملي ثانية",
  },

  data: {
    title: "البيانات اليومية",
    last24: "آخر 24 ساعة",
    download: "تنزيل",
    upload: "رفع",
    mb: "ميغابايت",
  },

  pages: {
    profile: "الملف الشخصي",
    plan: "الخطة",
    email: "البريد الإلكتروني",
    member: "عضو منذ",
    devices: "الأجهزة",
    network: "تحليل الشبكة",
    latency: "زمن الاستجابة",
    jitter: "التذبذب",
    loss: "فقدان الحزم",
    quality: "جودة الاتصال",
    excellent: "ممتازة",
  },

  settings: {
    title: "الإعدادات",
    appearance: "المظهر",
    dark: "الوضع المظلم",
    light: "الوضع الفاتح",
    startup: "التشغيل مع بدء النظام",
    killswitch: "مفتاح الإيقاف (Kill Switch)",
    autoconnect: "الاتصال التلقائي عند البدء",
    dns: "خادم DNS",
    dnsAuto: "تلقائي",
  },

  about: {
    title: "حول RikaVPN",
    developer: "المطوّر",
    link: "حساب المطوّر على منصة X",
    version: "الإصدار",
    desc: "RikaVPN تطبيق VPN سريع وخفيف مكتوب بلغة Rust، بواجهة عربية عصرية تدعم الوضع المظلم والفاتح. يوفّر اتصالًا آمنًا بسيرفرات في: اليابان، الصين، كوريا، فيتنام، أمريكا، وبريطانيا — مع المزيد قريبًا.",
    benefits: "لماذا RikaVPN؟",
    b1: "بروتوكول WireGuard الحديث — سرعة عالية واستقرار ممتاز.",
    b2: "بديل OpenVPN — توافق واسع مع مختلف الشبكات.",
    b3: "حجم صغير واستهلاك موارد منخفض.",
    b4: "خصوصية أولًا: لا نبيع بياناتك ولا نسجّل نشاطك.",
    b5: "واجهة عربية كاملة مع وضعين مظلم وفاتح.",
    credits: "شكر وتراخيص",
    creditMap: "خريطة العالم: Simple World Map — Al MacDonald / Fritz Lekschas (CC BY-SA 3.0).",
    creditFont: "خط Cairo: Google Fonts (SIL OFL 1.1).",
    creditFlags: "الأعلام: flagcdn.com (رخصة عامة).",
  },
};

export function formatDuration(total) {
  const s = Math.max(0, Math.floor(total));
  const hh = String(Math.floor(s / 3600)).padStart(2, "0");
  const mm = String(Math.floor((s % 3600) / 60)).padStart(2, "0");
  const ss = String(s % 60).padStart(2, "0");
  return `${hh}:${mm}:${ss}`;
}
