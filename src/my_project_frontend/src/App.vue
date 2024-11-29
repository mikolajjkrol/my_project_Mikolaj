<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let displasyChat = ref([]);

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const msg = target.querySelector('#msg').value;
  await my_project_backend.save_msg(msg)
  await getChat()
}

async function getChat() {
  displasyChat.value = await my_project_backend.get_chat()
}

getChat()
</script>

<template>
  <main class="container mx-auto"><div id="cont">
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <input id="msg" alt="msg" type="text" placeholder="Enter here" />
      <br><br>
      <button type="submit">Click Me!</button>
    </form>
    <br>
    <hr></hr>
    <br>
    <section id="displasyChat">
      <div v-for="msg in displasyChat">{{ msg }}</div>
    </section>
    <br>
    <a href="https://www.youtube.com/watch?v=frAhxXbLetk">KLIK</a>
  </div></main>
</template>