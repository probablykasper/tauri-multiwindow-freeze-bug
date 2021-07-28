<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'

  export function popup(msg: string) {
    invoke('error_popup', { msg })
  }

  async function newWindow() {
    await invoke('new_window')
      .then(() => {
        console.log('new_file success')
      })
      .catch(popup)
  }
</script>

<main>
  <div class="buttons">
    <button on:click={newWindow}>New Window</button>
  </div>
  <input type="text" />
</main>
