import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {ChatEntry} from "../data/chat-entry";

@Injectable({
  providedIn: 'root'
})
export class ChatService {

  public chatList: ChatEntry[] = []

  constructor(private http: HttpClient) {
    this.updateChatList()
  }

  public updateChatList() {
    this.http.get<ChatEntry[]>("/api/chat/list")
      .subscribe({
        next: value => this.chatList = value
      })
  }

  createChat(name: string) {
    this.http.post("/api/chat/create", {
      name: name
    }).subscribe({
      next: value => {
        this.updateChatList()
      }
    })
  }
}
