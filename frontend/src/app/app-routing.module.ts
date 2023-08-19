import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {HomeComponent} from "./home/home.component";
import {LoginComponent} from "./login/login.component";
import {notLoggedInGuard} from "./guard/not-logged-in.guard";
import {loggedInGuard} from "./guard/logged-in.guard";
import {DashboardComponent} from "./dashboard/dashboard.component";
import {CreateChatComponent} from "./dashboard/create-chat/create-chat.component";
import {ActiveChatComponent} from "./dashboard/active-chat/active-chat.component";

const routes: Routes = [
  {
    path: "",
    component: HomeComponent,
    children: [
      {
        path: "",
        component: LoginComponent,
        canActivate: [notLoggedInGuard],
      },
      {
        path: "dashboard",
        component: DashboardComponent,
        canActivate: [loggedInGuard],
        children: [
          {
            path: "chat/create",
            component: CreateChatComponent,
          },
          {
            path: "chat/:id",
            component: ActiveChatComponent
          }
        ]
      }
    ]
  },
  {
    path: "**",
    redirectTo: ""
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
