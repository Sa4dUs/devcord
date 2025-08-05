import { Component, inject, signal } from "@angular/core";
import { CommonModule } from "@angular/common";
import { ImageCroppedEvent, ImageCropperComponent } from "ngx-image-cropper";
import { MAT_DIALOG_DATA, MatDialogModule } from "@angular/material/dialog";
import { MatButtonModule } from "@angular/material/button";
import { HEIGHT, WIDTH } from "../../../../main-menuConstants";

export type CropperDialogData = {
    image: File;
};

export type CropperDialogResult = {
    blob: Blob;
    imageUrl: string;
};

@Component({
    selector: "app-cropper-dialog",
    standalone: true,
    imports: [
        CommonModule,
        ImageCropperComponent,
        MatButtonModule,
        MatDialogModule,
    ],
    templateUrl: "cropper-dialog.component.html",
})
export class CropperDialogComponent {
    data = inject(MAT_DIALOG_DATA) as { image: File };

    // Usa tus constantes directamente
    readonly cropWidth = WIDTH;
    readonly cropHeight = HEIGHT;
    readonly aspectRatio = Math.round((WIDTH / HEIGHT) * 100) / 100;

    result = signal<CropperDialogResult | undefined>(undefined);

    imageCropped(event: ImageCroppedEvent) {
        const { blob, objectUrl } = event;
        if (blob && objectUrl) {
            this.result.set({ blob, imageUrl: objectUrl });
        }
    }
}
