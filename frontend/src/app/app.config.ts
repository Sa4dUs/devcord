import {
    ApplicationConfig,
    provideBrowserGlobalErrorListeners,
    provideZonelessChangeDetection,
} from "@angular/core";
import { provideRouter } from "@angular/router";
//configuración cara al cliente, comportamiento general de angular (esas funciones tochas me las creo)
import { routes } from "./app.routes";
import {
    provideClientHydration,
    withEventReplay,
} from "@angular/platform-browser";
import { provideHttpClient } from "@angular/common/http";

export const appConfig: ApplicationConfig = {
    providers: [
        provideBrowserGlobalErrorListeners(),
        provideZonelessChangeDetection(),
        provideRouter(routes),
        provideClientHydration(withEventReplay()),
        provideHttpClient(),
    ],
};
