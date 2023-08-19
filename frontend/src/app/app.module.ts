import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { HomeComponent } from './home/home.component';
import {HttpClientModule} from "@angular/common/http";
import { LoginComponent } from './login/login.component';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import { DashboardComponent } from './dashboard/dashboard.component';
import { ChatListComponent } from './dashboard/chat-list/chat-list.component';
import { ChatListEntryComponent } from './dashboard/chat-list/chat-list-entry/chat-list-entry.component';
import { BarComponent } from './dashboard/bar/bar.component';
import { CreateChatComponent } from './dashboard/create-chat/create-chat.component';
import { ActiveChatComponent } from './dashboard/active-chat/active-chat.component';
import { MessagePromptComponent } from './dashboard/active-chat/message-prompt/message-prompt.component';

@NgModule({
  declarations: [
    AppComponent,
    HomeComponent,
    LoginComponent,
    DashboardComponent,
    ChatListComponent,
    ChatListEntryComponent,
    BarComponent,
    CreateChatComponent,
    ActiveChatComponent,
    MessagePromptComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
