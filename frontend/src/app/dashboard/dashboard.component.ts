import {Component, OnInit} from '@angular/core';
import {ChatMessage} from "../data/chat-message";
import {NotificationService} from "../service/notification.service";

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent implements OnInit {

  constructor(private notificationSubject: NotificationService) {
  }

  ngOnInit(): void {

  }
}
