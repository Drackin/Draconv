import { invoke } from "@tauri-apps/api/core"
import { type CompletedJob, type FileData } from "./types"
import file_exts from "@/assets/extensions.json";
import supported_types from "@/assets/supported.json";
import { useDialogs } from "./useDialogs";

export const useFileStore = defineStore('file', {
    state: () => ({
        files: [] as FileData[],
        convertedFiles: [] as CompletedJob[],
        isProcessing: false,
        error: null as { title: string, message: string } | null,
    }),

    actions: {
        async getFileData(path: string) {
            try {
                const fileInfo: FileData = await invoke("get_file_data", { path })

                const type = getFileType(fileInfo.file_extension)
                const supported = supported_types[type as keyof typeof supported_types]
                let isSupported = false
                let convertibles: string[] = []
                if (supported)
                    isSupported = supported.inputs.find(type => type === fileInfo.file_extension.toLowerCase()) ? true : false
                else {
                    useDialogs().isDialogOpen = true
                    return this.error = { title: "Unsupported File Type", message: `The file type "${fileInfo.file_extension}" is not supported.` }
                }

                convertibles = supported && supported.outputs ? supported.outputs.filter(ext => ext !== fileInfo.file_extension) : []

                this.files.push({
                    ...fileInfo,
                    full_path: path,
                    file_type: type,
                    selected_extension: "unselected",
                    isSupported,
                    convertibles,
                    conversionStatus: "idle"
                });

                this.error = null

                return {
                    ...fileInfo,
                    file_type: type,
                    selected_extension: "unselected"
                }
            } catch (e: any) {
                this.error = { title: "Error", message: e }

                return null
            }
        },
        deleteFile(id: string) {
            this.files = this.files.filter(file => file.id !== id)

            if (this.files.length === 0)
                this.$reset()
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