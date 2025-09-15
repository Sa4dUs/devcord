import { Routes } from "@angular/router";
import { RegisterPage } from "./features/auth/pages/register/register.page";
import { LogInPage } from "./features/auth/pages/log-in/log-in.page";
import { HomePage } from "./pages/home.page";
import { FriendshipRequestComponent } from "./features/friendship/friendship-request/friendship-request.component";
import { FriendshipSentComponent } from "./features/friendship/friendship-sent/friendship-sent.component";
import { FriendRequestsComponent } from "./features/friendship/received-friendship/received-friendship.component";
import { FriendshipFriendComponent } from "./features/friendship/friend/friend.component";
import { BlockComponent } from "./features/blocking/block/block.component";
import { UnblockComponent } from "./features/blocking/unblock/unblock.component";
import { UpdateUsernameComponent } from "./features/update/update.component";
import { MainMenuPage } from "./features/main-menu/pages/main-menu.page";
import { GroupPage } from "./features/groups/pages/group.page";
import { CallPage } from "./features/call/pages/call.page";

//TODO: @AlexGarciaPrada Friendships and Co are pages, components??
export const routes: Routes = [
    { path: "register", component: RegisterPage },
    { path: "login", component: LogInPage },
    { path: "home", component: HomePage },
    { path: "friendship-request", component: FriendshipRequestComponent },
    { path: "recieved-friendship", component: FriendRequestsComponent },
    { path: "friendship-sent", component: FriendshipSentComponent },
    { path: "friendship-friend", component: FriendshipFriendComponent },
    { path: "blocks-block", component: BlockComponent },
    { path: "blocks-unblock", component: UnblockComponent },
    { path: "update", component: UpdateUsernameComponent },
    { path: "main-menu", component: MainMenuPage },
    { path: "group/:groupId", component: GroupPage },
    //At the time there is only one call in each group
    { path: "group/:groupId/call", component: CallPage },

    { path: "", redirectTo: "home", pathMatch: "full" },
];
