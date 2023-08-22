import {Injectable, OnChanges, SimpleChanges} from '@angular/core';
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
    }).subscribe()
  }

  public getMessagesPaginated(chatId: number, count: number, offset: number) {
    return this.http.get<ChatMessage[]>(`/api/message/${chatId}?count=${count}&offset=${offset}`)
  }

  ngOnChanges(changes: SimpleChanges): void {
    console.log(changes)
  }
}
