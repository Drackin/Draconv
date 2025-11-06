import { invoke } from "@tauri-apps/api/core"
import { type FileData } from "./types"
import file_exts from "@/assets/extensions.json";

export const useFileStore = defineStore('file', {
    state: () => ({
        file_data: null as FileData | null,
        selected_extension: "unselected",
        conversionStatus: "idle" as "idle" | "processing" | "error" | "success" | "cancelled",
        error: null as { title: string, message: string } | null
    }),

    actions: {
        async getFileData(path: string) {
            this.$reset()
            try {
                const fileInfo: FileData = await invoke("get_file_name", { path })

                const type = getFileType(fileInfo.file_extension)

                this.file_data = { ...fileInfo, file_type: type }

                this.error = null

                return this.file_data
            } catch (e: any) {
                this.error = { title: "Invalid File Type", message: e }
                this.file_data = null

                return null
            }
        },
        deleteFile() {
            this.file_data = null
        }
    }
})

const getFileType = (extension: string): string => {
    const ext = extension.toLowerCase() as keyof typeof file_exts

    if(file_exts[ext] && file_exts[ext][0]) {
        return file_exts[ext][0] as string
    } else {
        return "." + ext
    }
}