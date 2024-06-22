<script>
  import Workbench from "./components/workbench.svelte";
</script>

<svelte:head>
  <link
    rel="stylesheet"
    href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"
  />
</svelte:head>

<div>
  <Workbench />
</div>

<style>
  div {
    display: flex;
    justify-content: center;
  }

  :global(body) {
    font-family: "Poppins", sans-serif;
    margin: 0;
  }
</style>
