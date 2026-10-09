<script>
  import ServerCard from "./ServerCard.svelte";
  import SpeedCard from "./SpeedCard.svelte";
  import DataCard from "./DataCard.svelte";
</script>

<div class="grid grid-cols-1 gap-7 xl:grid-cols-[1.05fr_1fr]">
  <ServerCard />
  <div class="flex flex-col gap-7">
    <SpeedCard />
    <DataCard />
  </div>
</div>
