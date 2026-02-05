import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { DashboardData } from '../types';

interface TaskState {
  dashboard: DashboardData | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  fetchDashboard: () => Promise<void>;
  quickCapture: (title: string) => Promise<void>;
  completeTask: (id: string) => Promise<void>;
  deferTask: (id: string) => Promise<void>;
  deleteTask: (id: string) => Promise<void>;
  startTask: (id: string) => Promise<void>;
}

export const useTaskStore = create<TaskState>((set) => ({
  dashboard: null,
  isLoading: false,
  error: null,

  fetchDashboard: async () => {
    set({ isLoading: true, error: null });
    try {
      const data = await invoke<DashboardData>('get_dashboard_data');
      set({ dashboard: data, isLoading: false });
    } catch (error) {
      set({ error: String(error), isLoading: false });
    }
  },

  quickCapture: async (title: string) => {
    try {
      await invoke('quick_capture', { title });
      // Refresh dashboard after capture
      const data = await invoke<DashboardData>('get_dashboard_data');
      set({ dashboard: data });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  startTask: async (id: string) => {
    try {
      await invoke('update_task_status', { id, status: 'now' });
      const data = await invoke<DashboardData>('get_dashboard_data');
      set({ dashboard: data });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  completeTask: async (id: string) => {
    try {
      await invoke('update_task_status', { id, status: 'done' });
      // Refresh dashboard (backend handles promotion of next task if needed)
      const data = await invoke<DashboardData>('get_dashboard_data');
      set({ dashboard: data });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  deferTask: async (id: string) => {
    try {
      await invoke('update_task_status', { id, status: 'waiting' });
      const data = await invoke<DashboardData>('get_dashboard_data');
      set({ dashboard: data });
    } catch (error) {
      set({ error: String(error) });
    }
  },

  deleteTask: async (id: string) => {
    try {
      await invoke('delete_task', { id });
      const data = await invoke<DashboardData>('get_dashboard_data');
      set({ dashboard: data });
    } catch (error) {
      set({ error: String(error) });
    }
  }
}));
