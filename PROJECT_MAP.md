# RikaVPN — PROJECT_MAP.md

> **Document type:** Living architecture map (source of truth)
> **Author:** LazySano — https://x.com/lazysanosky
> **Created:** 2026-10-09
> **Target:** Windows 10/11 x64 (desktop)
> **Language of UI:** Arabic only (RTL)
> **Status:** RELEASE READY (v0.1.0) — installers مبنية (1.4–2.0 MB)؛ النفق الحقيقي يحتاج `wintun.dll` + config + صلاحيات مدير

---

## 0. SCOPE BOUNDARY (قاعدة غير قابلة للتفاوض)

**In scope (يُبنى):**
- VPN client يتصل عبر **WireGuard** و **OpenVPN** باستخدام ملفات/مفاتيح **يملكها المستخدم** أو مصادر **مجانية رسمية مصرّح بها** (مثل حصص ProtonVPN Free / Windscribe Free / قوالب VPNGate العامة).
- بلدان v1: 🇯🇵 اليابان، 🇨🇳 الصين، 🇰🇷 كوريا، 🇻🇳 فيتنام، 🇺🇸 أمريكا، 🇬🇧 بريطانيا (قابلة للتوسّع).
- واجهة عربية RTL، وضعان مظلم/فاتح، إحصاءات حيّة، اختبار سرعة، صفحة «حول».

**Out of scope (مرفوض):**
- تجريف/تخمين مستخدمي سيرفرات لا نملك إذنًا بها.
- أي «حل ملتوٍ» لتجاوز خدمات مدفوعة أو أنظمة حماية.
- الاستخدام غير المصرّح به لأي مورد.

---

## 1. [TECH_STACK] — إصدارات مثبّتة (تحقّق 2026-10-09)

### Rust (backend / core)
| Crate | Pinned | الدور |
|---|---|---|
| `tauri` | `2.12.2` | نافذة + IPC + bundler |
| `tauri-build` | `2.7.1` | build script (أعلى مستقر — يتأخّر عن `tauri`) |
| `boringtun` | `0.7.1` | WireGuard userspace (Cloudflare) |
| `wintun` | `0.5.1` | ربط WinTun (TUN driver) |
| `tokio` | `1.53.2` | async runtime |
| `tracing` | `0.1.44` | logging واجهة |
| `tracing-appender` | `0.2.5` | كاتب non-blocking |
| `tracing-subscriber` | `0.3.x` *(يُثبّت في M0)* | طبقة التنسيق |
| `keyring` | `4.2.0` | تخزين آمن للمفاتيح (Windows Credential Manager) |
| `serde` / `serde_json` | `1.x` | تسلسل الإعدادات |
| `thiserror` | `2.x` | أخطاء مُصنّفة |
| `chrono` / `time` | `0.4.x` | المؤقّت والإحصاءات |

### Frontend (WebView2)
| NPM | Pinned | الدور |
|---|---|---|
| `@tauri-apps/cli` | `2.12.1` | dev/build |
| `@tauri-apps/api` | `2.12.2` | جسر JS↔Rust |
| `@tauri-apps/plugin-shell` | `2.4.1` | تشغيل OpenVPN كعملية |
| `svelte` | `5.57.2` | إطار UI |
| `vite` | `8.3.4` | bundler |
| `@sveltejs/vite-plugin-svelte` | `7.3.1` | ربط Svelte |
| `tailwindcss` | `4.3.3` | تنسيق + dark mode |

### Runtime / System
- **WebView2 Runtime** (مضمّن في Win11، يُنزَّل في الويندوز الأقدم).
- **MSVC Build Tools** (linker) + **rustup** — غير مثبّتين حاليًا ⚠.
- **WinTun driver** (يُوزَّع كـ DLL + driver، يحتاج elevation مرّة أولى).
- mục tiêu حجم الإخراج: **< 20 MB** (مثبت MSI/NSIS).

> ملاحظة: تم تجاهل كل حزمة deprecated؛ كل الأسماء أعلاه هي الحزم الرسمية النشطة.
> **الواجهة:** Svelte + Vite **SPA** (وليست SvelteKit) — تبسيطًا وتقليلًا للحجم حسب Simplicity First.

---

## 1.b [ENVIRONMENT] — مُتحقّق منه فعليًا (2026-10-09)

| المكوّن | الحالة |
|---|---|
| `rustc` / `cargo` | `1.99.0` — toolchain `stable-x86_64-pc-windows-msvc` (default) |
| رابط MSVC | Visual Studio Community 2026 (VC Tools `14.50.35717` + Windows SDK `10.0.26100.0`) |
| WebView2 Runtime | `154.0.4258.62` |
| Node / npm | `v24.18.1` / `11.6.1` |
| Node deps | مثبّتة، `0 vulnerabilities` (110 حزمة) |
| frontend build | `vite build` → JS `.js` ~26 kB (gzip 10.8 kB) + CSS ~9 kB |
| Tauri build | `tauri build --debug --no-bundle` ناجح → `src-tauri/target/debug/rika-vpn.exe` |
| Smoke test | تشغيل `rika-vpn.exe` بقي حيًّا 6s (النافذة تُفتح) ثم أُغلق نظيفًا |

---

## 2. [SYSTEM_FLOW]

### 2.1 رحلة المستخدم (User Journey — Verifiable)
1. **تشغيل التطبيق** → يقرأ آخر بروفايل/سيرفر → حالة `Idle`.
2. **اختيار بلد** (اليابان/الصين/كوريا/فيتنام/أمريكا/بريطانيا) → اختيار سيرفر/مدينة → تحميل `Profile`.
3. **زر Connect** → فحص الصلاحيات (elevation) → تشغيل `helper` الخدمي → إنشاء محوّل WinTun → ضبط IP/DNS/Routes → المصافحة (handshake) → حالة `Connected`.
4. **إحصاءات حيّة** → استطلاع rx/tx كل ثانية → دفع إلى الواجهة → رسوم Line/Bar + مؤقّت + استهلاك يومي.
5. **اختبار السرعة** → عيّنة Download/Upload → حساب Peak Mbps.
6. **Disconnect** → إزالة Routes/DNS → إغلاق المحوّل → `Idle`.
7. **السجلّات** → `tracing` غير حظري إلى ملف + عرض خفيف داخل الواجهة.

### 2.2 تدفّق البيانات (Data Flow — brief)
```
UI (Svelte)  --invoke-->  app/commands.rs  -->  core/ (domain, pure)
                                    |
                                    v
                            infra/ (wg_engine | ovpn_engine | wintun | net | store)
                                    |
                                    +--> OS (WinTun / routes / DNS / Credential Manager)
     <--events (connect-state, stats)--  Emitter
```

---

## 3. [ARCHITECTURE] — Domain-Driven، بلا Micro-files

```
RikaVPN/
├─ src-tauri/
│  ├─ Cargo.toml                 # الإصدارات المثبّتة أعلاه
│  ├─ build.rs
│  ├─ tauri.conf.json            # النافذة، الـ bundle، RTL title
│  └─ src/
│     ├─ main.rs                 # entry + Tauri builder + state init
│     ├─ logging.rs              # tracing non-blocking + مستوى السجل
│     ├─ app/
│     │  ├─ commands.rs          # كل #[tauri::command] (connect/disconnect/import/stats/...)
│     │  └─ state.rs             # AppState (Mutex<ConnectionManager>, stores)
│     ├─ core/                   # منطق نقي، بلا I/O
│     │  ├─ server.rs            # Country, Server, Protocol, ServerCatalog
│     │  ├─ tunnel.rs            # TunnelState, TunnelConfig, Tunnel trait
│     │  ├─ stats.rs             # StatsSnapshot + تجميع الاستهلاك
│     │  └─ profile.rs           # Profile + parse/validate .conf/.ovpn
│     └─ infra/                  # التطبيقات الفعلية (I/O)
│        ├─ wg_engine.rs         # boringtun + wintun glue
│        ├─ ovpn_engine.rs       # إدارة عملية openvpn (Child)
│        ├─ wintun.rs            # create/configure adapter
│        ├─ net.rs               # routes, DNS, public-ip check
│        ├─ win_service.rs       # helper مرتفع الصلاحيات
│        └─ store.rs             # JSON config + keyring
├─ src/                          # الواجهة (Svelte 5)
│  ├─ main.js
│  ├─ App.svelte                 # shell + routing (Dashboard/Profile/SpeedTest/Network/About/Settings)
│  ├─ app.css                    # Tailwind + RTL + theme tokens
│  ├─ i18n/ar.js                 # كل النصوص العربية في مكان واحد
│  └─ lib/
│     ├─ theme.js                # dark/light + persist
│     ├─ stores/                 # connection.js, servers.js, stats.js
│     ├─ charts/                 # LineChart.svelte, BarChart.svelte (SVG خالص)
│     └─ components/             # Sidebar, TopBar, ServerCard, WorldMap, AboutPanel, ...
├─ package.json                  # الإصدارات المثبّتة أعلاه
├─ vite.config.js
└─ PROJECT_MAP.md                # هذا الملف
```

**قواعد المعمارية:**
- `core/` لا يلمس النظام — قابل للاختبار وحدات (unit-testable).
- `infra/` هو المكان الوحيد الذي يستدعي OS/عمليات/Win32.
- لا تجريد لكود يُستخدم مرة واحدة؛ `Shared/Core` فقط للمنطق المتكرّر فعلًا (e.g. `net`, `stats`).
- الأخطاء تعبر الحدود كـ `thiserror` enum واحد.

---

## 4. [UI_SPEC] — مطابقة صور التصميم

**Shell:** Sidebar يمين/يسار مع RTL + TopBar (شعار `RikaVPN`, مبدّل الثيم، إشعارات).
**الشاشات:**
- **Dashboard:** بطاقة البلد (علم + مدينة + خريطة نقطية + موقع)، `Connected 01:21:36`، `IP`، Down/Up `MB/S`، زر `Disconnect` أرجواني دائري، بطاقة `Peak Speed 83.4 Mbps` + مخطط منحنى، بطاقة `Daily Data` + Dropdown `Last 24H` + مخطط أعمدة مزدوج.
- **My Profile:** بيانات المستخدم + الصورة.
- **Speed Test:** تشغيل قياس حقيقي (Ping/Down/Up).
- **Network Analyse:** إحصاءات جلسة مفصّلة.
- **About RikaVPN:** الوصف + الفوائد + **LazySano** + رابط `https://x.com/lazysanosky`.
- **Settings:** اللغة (عربي)، الثيم، التشغيل التلقائي، Kill-Switch، DNS.
- **Go PRO:** عنصر بصري فقط في v1 (بدون دفع فعلي) — يُخفى أو يُعطّل إن أردت.

**Design tokens:** بنفسجي أساسي `#6C5CE7` (تقريبي)، رمادي فاتح، زوايا دائرية كبيرة، ظلال ناعمة، خط عربي (Cairo / IBM Plex Sans Arabic).

---

## 5. [MILESTONES] — أهداف قابلة للتحقق (Verifiable Goals)

| # | الهدف | معيار النجاح (Definition of Done) |
|---|---|---|
| **M0** ✅ | بيئة التطوير | **منجز:** rustup + toolchain MSVC + WebView2 مثبّتة؛ scaffold Svelte SPA يُبنى؛ `rika-vpn.exe` يعمل |
| **M1** ✅ | قشرة الواجهة | **منجز:** Sidebar + TopBar + Dashboard (خريطة نقطية، مخطط Line/Bar، بطاقة سيرفر وزر اتصال يعمل) + 6 صفحات؛ RTL كامل؛ Dark/Light مُختبَران بلقطتي شاشة |
| **M2** ✅ | كتالوج السيرفرات | **منجز:** صفحة `ServersPage` (6 دول + أعلام + إبراز المحدّد)، استيراد `.conf/.ovpn` في الإعدادات + كشف البروتوكول، حفظ السيرفر/الملفات في `localStorage`؛ تحقّق بصري |
| **M3a** ✅ | نواة Rust | **منجز:** `core/profile.rs` (تحليل WireGuard/OpenVPN) + `#[cfg(test)]` 4 اختبارات ناجحة؛ `logging.rs` (tracing non-blocking)؛ أمر `parse_profile` عبر IPC؛ إضافة `boringtun 0.7.1` |
| **M3b** ✅ | محرّك WireGuard | **منجز+مُترجَم:** `infra/wg_engine.rs` (boringtun + WinTun + حلقة UDP/WinTun + مسارات/DNS/MTU) + فحص صلاحيات وإعادة تشغيل كمسؤول. يحتاج `wintun.dll` + config حقيقي للاختبار الحيّ |
| **M4** ✅ | الإحصاءات الحيّة | **منجز:** `connection_stats` يجسر rx/tx من المحرّك إلى الواجهة كل ثانية (م.ب/ث حقيقي) + المؤقّت + المخططات؛ fallback محاكاة للدول بلا config |
| **M5** ✅ | محرّك OpenVPN | **منجز+مُترجَم:** `infra/ovpn_engine.rs` يشغّل `openvpn.exe` بملف مؤقت ويتتبّع العملية؛ يتطلب وجود `openvpn.exe` (sidecar/PATH) |
| **M6** | صفحة «حول» | تحتوي بوضوح: مطوّر **LazySano** + الرابط `x.com/lazysanosky` |
| **M7** ✅ | التغليف | **منجز:** Release + مثبّتا NSIS/MSI؛ الحجم **1.42 / 1.98 MB** (أقل بكثير من 20MB)؛ يعمل على جهاز دون Rust |

---

## 6. [ORPHANS & PENDING] — نواقص ومسؤوليات مفتوحة

- **P1 ✅ (محسوم):** ثلاثة مسارات متاحة للمستخدم داخل التطبيق: (1) استيراد `.conf/.ovpn`، (2) قوالب مزوّدين مجانيين رسميين (ProtonVPN Free / VPNGate)، (3) مزوّد مخصّص يحدّده المستخدم بنفسه. القرار للمستخدم.
- **P2:** معمارية `helper` الخدمي (Windows Service مقابل Scheduled Task مقابل UAC لكل تشغيل) — يُحسم في M3.
- **P3 ✅ (محسوم):** `serde 1` + `tokio 1.53.2` + `tracing 0.1.44` + `tracing-appender 0.2.5` + `tracing-subscriber 0.3.x` + `thiserror 2` + `boringtun 0.7.1` — كلها مثبّتة ومُترجَمة.
- **P4 ✅ (محسوم):** الأصول مجانية ومُوثّقة في صفحة «حول»: الأعلام = flagcdn.com (رخصة عامة)، الخريطة = Simple World Map (CC BY-SA 3.0)، الخط = Cairo (SIL OFL 1.1).
- **P5:** Code Signing (يؤثّر على تحذير SmartScreen) — مؤجل.
- **P6:** عقد باقي البلدان/الميزات للتحديثات القادمة.
- **P7:** Kill-Switch و Split-Tunneling — مؤجلان لـ v2.

---

## 7. [ABOUT_CONTENT] — نص جاهز لصفحة «حول»

> **RikaVPN** — تطبيق VPN سريع وخفيف مكتوب بلغة **Rust**، بواجهة عربية عصرية تدعم الوضع المظلم والفاتح.
> يوفّر اتصالًا آمنًا بسيرفرات في: اليابان، الصين، كوريا، فيتنام، أمريكا، وبريطانيا — مع المزيد قريبًا.
> **تم تطويره بواسطة LazySano** — https://x.com/lazysanosky

---

## 8. [DISTRIBUTION & UPDATES] (2026-10-09)

- **Auto-update:** `tauri-plugin-updater 2.13.2` + `tauri-plugin-process 2.4.0`؛ التوقيع بمفتاح minisign (المفتاح العام داخل `tauri.conf.json > plugins.updater.pubkey`).
- **المصدر:** GitHub Releases → `latest.json` (يُنتجه `tauri-action` تلقائيًا).
- **المفتاح الخاص:** يُحفظ كسريًّا في GitHub Secrets باسم `TAURI_SIGNING_PRIVATE_KEY` وكلمة مروره `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`.
- **Workflow:** `.github/workflows/release.yml` — بناء/توقيع/نشر عند دفع وسم `v*`.
- **المخرجات المُتحقَّق منها محليًا:** `RikaVPN_0.1.0_x64-setup.exe` (2.07MB) + `.sig`، و`RikaVPN_0.1.0_x64_en-US.msi` (2.76MB) + `.sig`.
- **PENDING:** استبدال `GITHUB_USERNAME/rika-vpn` بمسار المستودع الحقيقي؛ وربط نظام الحسابات (Express/PocketBase) على `api.niral.me` + Google OAuth.

## 9. CHANGE LOG
- `2026-10-09` — إنشاء الملف، تثبيت الإصدارات، اعتماد المعمارية والميستونز.
- `2026-10-09` — **M0 منجز:** تثبيت Rust (1.99.0) + التحقق من MSVC/WebView2؛ scaffold من `create-tauri-app` مع تحويله إلى Svelte SPA؛ `npm install` + `vite build` + `tauri build --debug` خضراء؛ `rika-vpn.exe` يعمل. حُسم P1 (كل المسارات متاحة للمستخدم).
- `2026-10-09` — **M1 منجز:** بناء الهيكل الكامل (Svelte 5 runes) + i18n عربي + خط Cairo مُضمَّن + خريطة نقطية + مخططات SVG + لوحة تحكم و6 صفحات؛ إضافة `tauri-plugin-opener` 2.7.0 (رابط X يعمل)؛ تحقّق بصري بلقطتي شاشة (فاتح/مظلم). حُسم P4 (تراخيص الأصول).
- `2026-10-09` — **M2 منجز:** صفحة اختيار الدول + استيراد ملفات التعريف (`core` JS: `detectProtocol`) + حفظ دائم للسيرفر والملفات (`localStorage`)؛ ربط زر السيرفر بالصفحة؛ تحقّق بصري.
- `2026-10-09` — **M3a منجز:** نواة Rust (`core/profile.rs` + 4 اختبارات ناجحة)، `logging.rs` (tracing async)، أمر IPC `parse_profile`، ربطه بواجهة الاستيراد؛ إضافة `boringtun`. حُسم P3.
- `2026-10-09` — **M3b/M4/M5/M7 منجز:** محرّك WireGuard (boringtun+WinTun+مسارات)، إحصاءات حيّة، محرّك OpenVPN، فحص/رفع الصلاحيات؛ **Release مثبّتات 1.42/1.98MB**؛ 7 اختبارات Rust ناجحة. النفق الحيّ ينتظر `wintun.dll` + config.
- `2026-10-09` — **نظام الحسابات + الإشعارات + البيانات الحقيقية:** صفحة دخول/إنشاء حساب + خروج، لوحة إشعارات بالتفاصيل، دليل مساعدة، مراقبة شبكة حقيقية (Win32 GetIfTable2)، اختبار سرعة فعلي (Cloudflare) مع سجل آخر 5، تحليل شبكة فعلي (TCP probe)، إعدادات فعّالة (تشغيل تلقائي/DNS/Kill Switch)، إزالة PRO، تعريب كامل. أُضيف `windows-sys` + أوامر جديدة.
- `2026-10-09` — **التحديث الذاتي:** `tauri-plugin-updater` + `process`، مفتاح توقيع minisign، `createUpdaterArtifacts`، وورك‑فلو GitHub لنشر الإصدارات؛ تحقّق من إنتاج `.sig` محليًا.
