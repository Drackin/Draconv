export type FileData = {
    id: string
    dir_path: string
    full_file_name: string
    file_extension: string
    file_name: string
    file_type: string,
    full_path: string
    selected_extension: string
    isSupported: boolean,
    convertibles: string[]
    conversionStatus: "idle" | "queued" | "processing" | "failed" | "success" | "cancelled"
    progress?: number
}

export type CompletedJob = {
    id: string
    total_time: number
    input_file: string
    new_file_path: string
}

export type ConversionOutput = {
    total_time: number
    new_file_path: string
}