export type TaskStatus = 'now' | 'next' | 'waiting' | 'someday' | 'done';

export interface Task {
  id: string;
  title: string;
  status: TaskStatus;
  context?: string;
  scheduled_for?: number;
  completed_at?: number;
  created_at: number;
  updated_at: number;
  original_input?: string;
  source: string;
  tags?: string;
  sync_version: number;
}

export interface DashboardData {
  now_task: Task | null;
  next_tasks: Task[];
  waiting_tasks: Task[];
  review_due_in_days: number;
}
