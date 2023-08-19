import {Component, OnInit} from '@angular/core';
import {ChatMessage} from "../data/chat-message";

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent implements OnInit {
  ngOnInit(): void {
    let es = new EventSource("/api/notification/subscribe")

    es.onmessage = (msg) => {
      let data: ChatMessage = JSON.parse(msg.data)
      console.log(data)
    }
  }
}
