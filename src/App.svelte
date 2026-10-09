<script>
  import { onMount } from "svelte";
  import LoginPage from "$lib/components/LoginPage.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import TopBar from "$lib/components/TopBar.svelte";
  import Dashboard from "$lib/components/Dashboard.svelte";
  import ServersPage from "$lib/pages/ServersPage.svelte";
  import ProfilePage from "$lib/pages/ProfilePage.svelte";
  import SpeedTestPage from "$lib/pages/SpeedTestPage.svelte";
  import NetworkPage from "$lib/pages/NetworkPage.svelte";
  import AboutPage from "$lib/pages/AboutPage.svelte";
  import SettingsPage from "$lib/pages/SettingsPage.svelte";
  import {
    app,
    initTheme,
    tick,
    refreshElevated,
    maybeAutoconnect,
    pollNetwork,
  } from "$lib/stores/app.svelte.js";
  import { auth } from "$lib/stores/auth.svelte.js";
  import { checkForUpdates } from "$lib/stores/updater.svelte.js";

  const routes = {
    dashboard: Dashboard,
    servers: ServersPage,
    profile: ProfilePage,
    speed: SpeedTestPage,
    network: NetworkPage,
    about: AboutPage,
    settings: SettingsPage,
  };

  const Page = $derived(routes[app.page] ?? Dashboard);

  onMount(() => {
    initTheme();
    refreshElevated();
    pollNetwork();
    const timer = setInterval(tick, 1000);
    return () => clearInterval(timer);
  });

  let autoconnectTried = $state(false);
  $effect(() => {
    if (auth.session && !autoconnectTried) {
      autoconnectTried = true;
      maybeAutoconnect();
      checkForUpdates(true);
    }
  });
</script>

{#if !auth.session}
  <LoginPage />
{:else}
  <div class="relative flex h-screen w-screen overflow-hidden bg-surface-2 text-ink">
    <div class="pointer-events-none absolute inset-0 opacity-80">
      <div class="absolute -top-24 -start-24 h-72 w-72 rounded-full bg-brand/20 blur-3xl"></div>
      <div class="absolute bottom-0 end-[30%] h-72 w-72 rounded-full bg-danger/10 blur-3xl"></div>
    </div>

    <Sidebar />

    <div class="relative z-10 flex min-w-0 flex-1 flex-col">
      <TopBar />
      <main class="flex-1 overflow-y-auto px-7 pb-10 pt-2">
        <Page />
      </main>
    </div>
  </div>
{/if}
