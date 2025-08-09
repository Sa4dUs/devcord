import { Component, EventEmitter, Output, inject, signal } from "@angular/core";
import { CommonModule } from "@angular/common";
import { MatDialog, MatDialogModule } from "@angular/material/dialog";
import { MatButtonModule } from "@angular/material/button";
import { filter } from "rxjs/operators";
import {
    CropperDialogComponent,
    CropperDialogResult,
} from "./image-control/cropper-dialog/cropper-dialog.component";

@Component({
    selector: "bubble-container-background",
    standalone: true,
    imports: [CommonModule, MatDialogModule, MatButtonModule],
    templateUrl: "bubble-container-background.component.html",
    styles: [
        `
            button {
                margin-bottom: 16px;
            }
        `,
    ],
})
export class BubbleContainerBackground {
    dialog = inject(MatDialog);

    croppedImage = signal<string | undefined>(undefined);

    @Output() backgroundSelected = new EventEmitter<string>();

    fileChangeEvent(event: Event) {
        const input = event.target as HTMLInputElement;
        const file = input.files?.[0];
        if (!file) return;

        this.openCropperDialog(file, 400, 400);
    }

    openCropperDialog(file: File, width: number, height: number) {
        const dialogRef = this.dialog.open(CropperDialogComponent, {
            data: { image: file, width, height },
            width: "500px",
        });

        dialogRef
            .afterClosed()
            .pipe(filter((result): result is CropperDialogResult => !!result))
            .subscribe((result) => {
                const reader = new FileReader();
                reader.onloadend = () => {
                    const base64data = reader.result as string;
                    this.croppedImage.set(base64data);
                    this.backgroundSelected.emit(base64data);
                };
                reader.readAsDataURL(result.blob);
            });
    }
    removeBackground() {
        this.backgroundSelected.emit("");
    }

    changeBackgroundColor(event: Event) {
        const input = event.target as HTMLInputElement;
        const color = input.value;
        this.backgroundSelected.emit(color);
    }
}
