import {
    Component,
    EventEmitter,
    Input,
    NgZone,
    Output,
    computed,
    effect,
    inject,
    signal,
} from "@angular/core";
import { CommonModule } from "@angular/common";
import { MatDialog, MatDialogModule } from "@angular/material/dialog";
import { MatButtonModule } from "@angular/material/button";
import {
    CropperDialogComponent,
    CropperDialogResult,
} from "./cropper-dialog/cropper-dialog.component";
import { filter } from "rxjs/operators";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { HEIGHT, WIDTH } from "../../../main-menuConstants";

@Component({
    selector: "app-image-control",
    standalone: true,
    imports: [
        CommonModule,
        MatDialogModule,
        MatButtonModule,
        MatProgressSpinnerModule,
    ],
    templateUrl: "image-control.component.html",
    styleUrl: "image-control.component.scss",
})
export class ImageControlComponent {
    imageWidth = signal(WIDTH);
    @Input({ required: true }) set width(val: number) {
        this.imageWidth.set(val);
    }

    imageHeight = signal(HEIGHT);
    @Input({ required: true }) set height(val: number) {
        this.imageHeight.set(val);
    }

    imagePath = signal("");
    @Input({ required: true }) set path(val: string) {
        this.imagePath.set(val);
        const saved = localStorage.getItem(val);
        if (saved) {
            this.croppedImageURL.set(saved);
        }
    }

    placeholder = computed(
        () => `https://placehold.co/${this.imageWidth()}x${this.imageHeight()}`,
    );

    croppedImageURL = signal<string | undefined>(undefined);

    imageSource = computed(() => {
        return this.croppedImageURL() ?? this.placeholder();
    });

    uploading = signal(false);

    dialog = inject(MatDialog);
    zone = inject(NgZone);

    @Output() imageReady = new EventEmitter<string>();

    constructor() {
        effect(() => {
            if (this.croppedImageURL()) {
                this.imageReady.emit(this.croppedImageURL());
            }
        });
    }

    fileSelected(event: Event) {
        const input = event.target as HTMLInputElement;
        const file = input?.files?.[0];
        if (file) {
            const dialogRef = this.dialog.open(CropperDialogComponent, {
                data: {
                    image: file,
                    width: this.imageWidth(),
                    height: this.imageHeight(),
                },
                width: "500px",
            });

            dialogRef
                .afterClosed()
                .pipe(filter((result) => !!result))
                .subscribe((result: CropperDialogResult) => {
                    this.uploadImage(result.blob);
                });
        }
    }

    async uploadImage(blob: Blob) {
        this.uploading.set(true);

        const reader = new FileReader();
        reader.onloadend = () => {
            const base64data = reader.result as string;
            localStorage.setItem(this.imagePath(), base64data);
            this.croppedImageURL.set(base64data);
            this.uploading.set(false);
        };
        reader.readAsDataURL(blob);
    }
}
