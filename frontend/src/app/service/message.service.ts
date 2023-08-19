import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {ChatMessage} from "../data/chat-message";

@Injectable({
  providedIn: 'root'
})
export class MessageService {

  constructor(private http: HttpClient) { }

  public sendMessage(chatId: number, message: string) {
    this.http.post(`/api/message/${chatId}`, {
      message: message
    }).subscribe({
      next: value => console.log(value)
    })
  }

  public getAllMessages(chatId: number) {
    return this.http.get<ChatMessage[]>("/api/message/")
  }
}
