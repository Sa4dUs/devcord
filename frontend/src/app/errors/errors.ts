import { HttpClient, HttpErrorResponse } from "@angular/common/http";
import { Injectable } from "@angular/core";
type ErrorMap = Map<number, string>;

@Injectable({
    providedIn: "root",
})
export class ErrorsHandling {
    private errorMaps = new Map<string, ErrorMap>();

    constructor(private http: HttpClient) {
        this.loadErrors();
    }

    private loadErrors() {
        this.http
            .get<
                Record<string, Record<string, string>>
            >("assets/errors-codes.json")
            .subscribe((data) => {
                for (const [context, codes] of Object.entries(data)) {
                    const map = new Map<number, string>();
                    for (const [code, message] of Object.entries(codes)) {
                        map.set(Number(code), message);
                    }
                    this.errorMaps.set(context, map);
                }
            });
    }
    getErrorMessage(context: string, error: HttpErrorResponse): string {
        const code = error.status;

        if (this.errorMaps.size === 0) {
            console.warn("Error maps not loaded yet");
            return `Unknown error: (${code})`;
        }

        const contextMap = this.errorMaps.get(context);
        if (contextMap?.has(code)) return contextMap.get(code)!;

        const defaultMap = this.errorMaps.get("default");
        if (defaultMap?.has(code)) return defaultMap.get(code)!;

        return `Unknown error: (${error})`;
    }
}
