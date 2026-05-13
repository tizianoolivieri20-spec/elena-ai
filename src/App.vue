<template>
  <div id="elena-app" class="app-container">
    <!-- Header -->
    <header class="app-header">
      <div class="header-content">
        <h1>Elena AI Assistant</h1>
        <p class="subtitle">Created by Tiziano O.</p>
      </div>
      <div class="mood-indicator" :class="`mood-${currentMood}`">
        <span class="mood-label">Your mood: {{ currentMood }}</span>
      </div>
    </header>

    <!-- Main Container -->
    <div class="main-container">
      <!-- Chat Area -->
      <div class="chat-area">
        <div class="messages-container">
          <div 
            v-for="msg in messages" 
            :key="msg.id"
            :class="['message', msg.role]"
          >
            <div class="message-content">
              <div class="message-text">{{ msg.content }}</div>
              <div class="message-time">{{ msg.timestamp }}</div>
            </div>
          </div>
          <div v-if="isLoading" class="message assistant loading">
            <div class="typing-indicator">
              <span></span>
              <span></span>
              <span></span>
            </div>
          </div>
        </div>

        <!-- Input Area -->
        <div class="input-area">
          <div class="input-wrapper">
            <textarea
              v-model="inputMessage"
              placeholder="Talk to Elena... (Shift+Enter for new line)"
              class="message-input"
              @keydown.enter.shift.prevent="sendMessage"
              @keydown.enter.prevent="!inputMessage.includes('\n') && sendMessage()"
            ></textarea>
            <button 
              @click="sendMessage" 
              :disabled="!inputMessage.trim() || isLoading"
              class="send-button"
            >
              Send
            </button>
          </div>
          <div class="input-actions">
            <button @click="clearChat" class="action-button">Clear Chat</button>
            <button @click="toggleMoodSelector" class="action-button">Set Mood</button>
          </div>
        </div>
      </div>

      <!-- Sidebar -->
      <aside class="sidebar">
        <div class="sidebar-section">
          <h3>Quick Actions</h3>
          <button class="sidebar-button">📄 Upload PDF</button>
          <button class="sidebar-button">🖼️ Upload Slide</button>
          <button class="sidebar-button">📊 Analyze Data</button>
        </div>

        <div class="sidebar-section">
          <h3>Mood Settings</h3>
          <div v-if="showMoodSelector" class="mood-selector">
            <button 
              v-for="mood in moods" 
              :key="mood"
              @click="setMood(mood)"
              :class="['mood-button', { active: currentMood === mood }]"
            >
              {{ mood }}
            </button>
          </div>
        </div>

        <div class="sidebar-section">
          <h3>Conversation Info</h3>
          <p class="info-text">Messages: {{ messages.length }}</p>
          <p class="info-text">Status: {{ isLoading ? 'Thinking...' : 'Ready' }}</p>
        </div>
      </aside>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri'
import { ref, computed, nextTick } from 'vue'

export default {
  name: 'App',
  setup() {
    const messages = ref([])
    const inputMessage = ref('')
    const isLoading = ref(false)
    const currentMood = ref('neutral')
    const showMoodSelector = ref(false)
    const moods = ['neutral', 'angry', 'sad', 'excited', 'confused']

    const sendMessage = async () => {
      if (!inputMessage.value.trim() || isLoading.value) return

      const userMessage = inputMessage.value.trim()
      inputMessage.value = ''
      isLoading.value = true

      try {
        const response = await invoke('send_message', { message: userMessage })
        
        // Fetch updated conversation
        const updatedMessages = await invoke('get_conversation')
        messages.value = updatedMessages
        
        // Scroll to bottom
        await nextTick()
        const container = document.querySelector('.messages-container')
        if (container) {
          container.scrollTop = container.scrollHeight
        }
      } catch (error) {
        console.error('Error sending message:', error)
        alert('Error: ' + error)
      } finally {
        isLoading.value = false
      }
    }

    const clearChat = async () => {
      if (confirm('Clear all messages?')) {
        await invoke('clear_conversation')
        messages.value = []
      }
    }

    const setMood = async (mood) => {
      currentMood.value = mood
      await invoke('set_user_mood', { mood })
      showMoodSelector.value = false
    }

    const toggleMoodSelector = () => {
      showMoodSelector.value = !showMoodSelector.value
    }

    // Load initial conversation
    const loadConversation = async () => {
      try {
        messages.value = await invoke('get_conversation')
      } catch (error) {
        console.error('Error loading conversation:', error)
      }
    }

    loadConversation()

    return {
      messages,
      inputMessage,
      isLoading,
      currentMood,
      showMoodSelector,
      moods,
      sendMessage,
      clearChat,
      setMood,
      toggleMoodSelector,
    }
  }
}
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

#elena-app {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Helvetica Neue', sans-serif;
  color: #333;
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.app-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 20px 30px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.header-content h1 {
  font-size: 28px;
  margin-bottom: 5px;
}

.subtitle {
  font-size: 13px;
  opacity: 0.9;
}

.mood-indicator {
  padding: 8px 16px;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.2);
  font-size: 13px;
  font-weight: 600;
}

.mood-label {
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.main-container {
  display: flex;
  flex: 1;
  gap: 0;
  overflow: hidden;
}

.chat-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
  border-right: 1px solid #e0e0e0;
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 20px 30px;
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.message {
  display: flex;
  animation: slideIn 0.3s ease-out;
}

.message.user {
  justify-content: flex-end;
}

.message.assistant {
  justify-content: flex-start;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.message-content {
  max-width: 70%;
  padding: 12px 16px;
  border-radius: 12px;
  word-wrap: break-word;
}

.message.user .message-content {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-bottom-right-radius: 4px;
}

.message.assistant .message-content {
  background: #f0f0f0;
  color: #333;
  border-bottom-left-radius: 4px;
}

.message-text {
  margin-bottom: 5px;
  line-height: 1.5;
}

.message-time {
  font-size: 11px;
  opacity: 0.7;
  margin-top: 5px;
}

.message.user .message-time {
  opacity: 0.6;
}

.message.loading {
  justify-content: flex-start;
}

.typing-indicator {
  display: flex;
  gap: 4px;
  padding: 12px 16px;
}

.typing-indicator span {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #667eea;
  animation: typing 1.4s infinite;
}

.typing-indicator span:nth-child(2) {
  animation-delay: 0.2s;
}

.typing-indicator span:nth-child(3) {
  animation-delay: 0.4s;
}

@keyframes typing {
  0%, 60%, 100% {
    opacity: 0.5;
    transform: translateY(0);
  }
  30% {
    opacity: 1;
    transform: translateY(-10px);
  }
}

.input-area {
  padding: 20px 30px;
  background: #f9f9f9;
  border-top: 1px solid #e0e0e0;
}

.input-wrapper {
  display: flex;
  gap: 12px;
  margin-bottom: 12px;
}

.message-input {
  flex: 1;
  padding: 12px 16px;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-family: inherit;
  font-size: 14px;
  resize: none;
  max-height: 120px;
  focus: {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }
}

.message-input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.send-button {
  padding: 12px 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.send-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.send-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.input-actions {
  display: flex;
  gap: 12px;
}

.action-button {
  flex: 1;
  padding: 10px;
  background: white;
  border: 1px solid #ddd;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s;
}

.action-button:hover {
  background: #f5f5f5;
  border-color: #667eea;
}

.sidebar {
  width: 280px;
  background: #f9f9f9;
  padding: 20px;
  overflow-y: auto;
  border-left: 1px solid #e0e0e0;
}

.sidebar-section {
  margin-bottom: 25px;
}

.sidebar-section h3 {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 12px;
  color: #667eea;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.sidebar-button {
  display: block;
  width: 100%;
  padding: 12px;
  margin-bottom: 8px;
  background: white;
  border: 1px solid #ddd;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.sidebar-button:hover {
  background: #667eea;
  color: white;
  border-color: #667eea;
}

.mood-selector {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.mood-button {
  padding: 10px;
  background: white;
  border: 2px solid #ddd;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s;
  text-transform: capitalize;
}

.mood-button:hover {
  border-color: #667eea;
}

.mood-button.active {
  background: #667eea;
  color: white;
  border-color: #667eea;
}

.info-text {
  font-size: 13px;
  color: #666;
  margin-bottom: 8px;
}

/* Mood color indicators */
.mood-neutral {
  background: rgba(100, 100, 100, 0.1);
  color: #666;
}

.mood-angry {
  background: rgba(255, 76, 76, 0.1);
  color: #ff4c4c;
}

.mood-sad {
  background: rgba(100, 150, 255, 0.1);
  color: #6496ff;
}

.mood-excited {
  background: rgba(255, 193, 7, 0.1);
  color: #ffc107;
}

.mood-confused {
  background: rgba(255, 152, 0, 0.1);
  color: #ff9800;
}

/* Scrollbar styling */
.messages-container::-webkit-scrollbar,
.sidebar::-webkit-scrollbar {
  width: 8px;
}

.messages-container::-webkit-scrollbar-track,
.sidebar::-webkit-scrollbar-track {
  background: transparent;
}

.messages-container::-webkit-scrollbar-thumb,
.sidebar::-webkit-scrollbar-thumb {
  background: #ccc;
  border-radius: 4px;
}

.messages-container::-webkit-scrollbar-thumb:hover,
.sidebar::-webkit-scrollbar-thumb:hover {
  background: #999;
}

@media (max-width: 1024px) {
  .sidebar {
    width: 200px;
    padding: 15px;
  }

  .message-content {
    max-width: 80%;
  }
}

@media (max-width: 768px) {
  .main-container {
    flex-direction: column;
  }

  .sidebar {
    width: 100%;
    border-left: none;
    border-top: 1px solid #e0e0e0;
    padding: 15px;
    max-height: 150px;
    display: flex;
    gap: 20px;
  }

  .sidebar-section {
    margin-bottom: 0;
    flex: 1;
  }

  .chat-area {
    border-right: none;
  }
}
</style>
