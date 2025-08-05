import { Component, Inject, inject, PLATFORM_ID, signal } from "@angular/core";
import { MatDialogModule } from "@angular/material/dialog";
import { HEIGHT, WIDTH } from "../../../main-menu/main-menuConstants";
import { CropperDialogResult } from "../../../main-menu/bubble-container/bubble-container-background/image-control/cropper-dialog/cropper-dialog.component";
import { CommonModule } from "@angular/common";
import { MatButtonModule } from "@angular/material/button";
import { HttpClient } from "@angular/common/http";
import { SERVER_ROUTE } from "../../../../environment/environment.secret";
import { ErrorsHandling } from "../../../errors/errors";
import { Router } from "@angular/router";
import { FriendListComponent } from "./friend-list/friend-list.component";

const context = "group-creation";

interface FriendList {
    username: string;
    created_at: string;
}

@Component({
    selector: "group-creation",
    standalone: true,
    imports: [
        CommonModule,
        MatButtonModule,
        MatDialogModule,
        FriendListComponent,
    ],
    templateUrl: "./friend-selector-dialog.component.html",
    styleUrls: ["./friend-selector-dialog.component.scss"],
})
export class FriendSelectorDialogComponent {
    private router = inject(Router);

    readonly cropWidth = WIDTH;
    readonly cropHeight = HEIGHT;
    readonly aspectRatio = Math.round((WIDTH / HEIGHT) * 100) / 100;

    result = signal<CropperDialogResult | undefined>(undefined);
    membersId: string[] | undefined;
    readonly requests = signal<FriendList[]>([]);
    readonly loading = signal(true);
    readonly error = signal<string | null>(null);

    constructor(
        private http: HttpClient,
        @Inject(PLATFORM_ID) private platformId: object,
        private errorsMap: ErrorsHandling,
    ) {}
    onSubmitMembersNewGroup(): void {
        this.http
            .post<{
                groupId: string;
            }>(SERVER_ROUTE + "/api/group/create", {
                membersId: this.membersId,
            })
            .subscribe({
                next: (data) => {
                    console.log(data);
                    this.router.navigate(["/main-menu"]);
                },
                error: (error) => {
                    console.error(
                        this.errorsMap.getErrorMessage(context, error),
                    );
                },
            });
    }
}
