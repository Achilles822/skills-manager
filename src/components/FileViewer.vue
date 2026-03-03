<script setup lang="ts">
import { ref, computed, watch } from "vue"
import { useI18n } from "vue-i18n"
import MarkdownIt from "markdown-it"
import hljs from "highlight.js/lib/core"
import javascript from "highlight.js/lib/languages/javascript"
import typescript from "highlight.js/lib/languages/typescript"
import python from "highlight.js/lib/languages/python"
import shell from "highlight.js/lib/languages/shell"
import json from "highlight.js/lib/languages/json"
import yaml from "highlight.js/lib/languages/yaml"
import markdown from "highlight.js/lib/languages/markdown"
import css from "highlight.js/lib/languages/css"
import xml from "highlight.js/lib/languages/xml"
import rust from "highlight.js/lib/languages/rust"
import NeuButton from "@/components/NeuButton.vue"
import type { FileContent } from "@/composables/useSkillFiles"

hljs.registerLanguage("javascript", javascript)
hljs.registerLanguage("typescript", typescript)
hljs.registerLanguage("python", python)
hljs.registerLanguage("shell", shell)
hljs.registerLanguage("bash", shell)
hljs.registerLanguage("json", json)
hljs.registerLanguage("yaml", yaml)
hljs.registerLanguage("markdown", markdown)
hljs.registerLanguage("css", css)
hljs.registerLanguage("xml", xml)
hljs.registerLanguage("html", xml)
hljs.registerLanguage("rust", rust)

const { t } = useI18n()

const md = new MarkdownIt({
  highlight(str: string, lang: string) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value
      } catch { /* fallback */ }
    }
    return ""
  },
})

const props = defineProps<{
  fileName: string | null
  filePath: string | null
  fileContent: FileContent | null
  loading: boolean
  isEditing: boolean
}>()

const emit = defineEmits<{
  save: [filePath: string, content: string]
  cancelEdit: []
}>()

const editContent = ref("")

watch(() => props.fileContent, (fc) => {
  if (fc && !fc.is_binary) {
    editContent.value = fc.content
  }
})

watch(() => props.filePath, () => {
  editContent.value = props.fileContent?.content || ""
})

const ext = computed(() => {
  if (!props.fileName) return ""
  return props.fileName.split(".").pop()?.toLowerCase() || ""
})

const isMarkdown = computed(() => ext.value === "md")

const highlightLang = computed(() => {
  const langMap: Record<string, string> = {
    js: "javascript",
    mjs: "javascript",
    cjs: "javascript",
    ts: "typescript",
    tsx: "typescript",
    jsx: "javascript",
    py: "python",
    sh: "shell",
    bash: "bash",
    zsh: "shell",
    json: "json",
    yaml: "yaml",
    yml: "yaml",
    md: "markdown",
    css: "css",
    html: "html",
    xml: "xml",
    svg: "xml",
    rs: "rust",
    toml: "yaml",
  }
  return langMap[ext.value] || null
})

const renderedMarkdown = computed(() => {
  if (!props.fileContent || props.fileContent.is_binary) return ""
  return md.render(props.fileContent.content)
})

const highlightedCode = computed(() => {
  if (!props.fileContent || props.fileContent.is_binary || !highlightLang.value) return ""
  try {
    return hljs.highlight(props.fileContent.content, { language: highlightLang.value }).value
  } catch {
    return ""
  }
})

function handleSave() {
  if (props.filePath) {
    emit("save", props.filePath, editContent.value)
  }
}
</script>

<template>
  <div class="file-viewer">
    <div v-if="loading" class="file-viewer__loading">{{ t('skill.loading') }}</div>

    <template v-else-if="fileContent">
      <div v-if="fileContent.is_binary" class="file-viewer__binary">
        {{ t('skill.binaryFileHint') }}
      </div>

      <template v-else-if="isEditing">
        <textarea
          v-model="editContent"
          class="file-viewer__textarea"
          spellcheck="false"
        />
        <div class="file-viewer__actions">
          <NeuButton @click="emit('cancelEdit')">{{ t('skill.cancel') }}</NeuButton>
          <NeuButton :disabled="!editContent" @click="handleSave">{{ t('skill.save') }}</NeuButton>
        </div>
      </template>

      <template v-else>
        <div v-if="isMarkdown" class="file-viewer__markdown" v-html="renderedMarkdown" />
        <div v-else-if="highlightLang" class="file-viewer__code">
          <pre><code v-html="highlightedCode" /></pre>
        </div>
        <div v-else class="file-viewer__plain">
          <pre>{{ fileContent.content }}</pre>
        </div>
      </template>
    </template>

    <div v-else class="file-viewer__empty">
      {{ t('skill.selectPrompt') }}
    </div>
  </div>
</template>

<style scoped>
.file-viewer {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.file-viewer__loading {
  color: var(--neu-text-muted);
  font-size: 0.9rem;
  padding: 1rem;
}

.file-viewer__binary {
  color: var(--neu-text-muted);
  font-size: 0.88rem;
  padding: 2rem;
  text-align: center;
}

.file-viewer__empty {
  color: var(--neu-text-muted);
  font-size: 0.88rem;
  padding: 2rem;
  text-align: center;
}

.file-viewer__textarea {
  flex: 1;
  min-height: 200px;
  padding: 0.875rem;
  font-family: ui-monospace, "Cascadia Code", "Source Code Pro", Menlo, monospace;
  font-size: 0.85rem;
  line-height: 1.5;
  background: var(--neu-bg);
  color: var(--neu-text);
  border: 2px solid transparent;
  border-radius: var(--neu-radius-sm);
  box-shadow: var(--neu-shadow-in);
  resize: vertical;
  transition: border-color var(--neu-transition);
}

.file-viewer__textarea:focus {
  outline: none;
  border-color: var(--neu-accent);
}

.file-viewer__actions {
  display: flex;
  gap: 0.5rem;
  margin-top: 0.5rem;
  padding: 2px;
}

.file-viewer__markdown {
  flex: 1;
  overflow-y: auto;
  font-size: 0.88rem;
  line-height: 1.6;
}

.file-viewer__markdown :deep(h1),
.file-viewer__markdown :deep(h2),
.file-viewer__markdown :deep(h3) {
  margin-top: 1.25em;
  margin-bottom: 0.5em;
}

.file-viewer__markdown :deep(p) {
  margin: 0.5em 0;
}

.file-viewer__markdown :deep(code) {
  background: rgba(209, 217, 230, 0.4);
  padding: 0.15em 0.4em;
  border-radius: 4px;
  font-size: 0.88em;
}

.file-viewer__markdown :deep(pre) {
  background: rgba(209, 217, 230, 0.25);
  padding: 0.875rem;
  border-radius: var(--neu-radius-sm);
  overflow-x: auto;
}

.file-viewer__markdown :deep(pre code) {
  background: none;
  padding: 0;
}

.file-viewer__code {
  flex: 1;
  overflow: auto;
}

.file-viewer__code pre {
  margin: 0;
  padding: 0.875rem;
  background: rgba(209, 217, 230, 0.18);
  border-radius: var(--neu-radius-sm);
  overflow-x: auto;
}

.file-viewer__code code {
  font-family: ui-monospace, "Cascadia Code", "Source Code Pro", Menlo, monospace;
  font-size: 0.83rem;
  line-height: 1.55;
}

.file-viewer__plain {
  flex: 1;
  overflow: auto;
}

.file-viewer__plain pre {
  margin: 0;
  padding: 0.875rem;
  font-family: ui-monospace, "Cascadia Code", "Source Code Pro", Menlo, monospace;
  font-size: 0.83rem;
  line-height: 1.55;
  white-space: pre-wrap;
  word-break: break-all;
}
</style>
