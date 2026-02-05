import { ReactNode } from 'react';

interface LayoutProps {
  children: ReactNode;
  headerContent?: ReactNode;
}

export function Layout({ children, headerContent }: LayoutProps) {
  return (
    <div className="min-h-screen bg-stone-50 text-stone-900 font-sans flex flex-col">
      {/* Header */}
      <header className="h-14 border-b border-stone-200 bg-white flex items-center justify-between px-6 sticky top-0 z-10 shadow-sm">
        <div className="font-bold text-lg tracking-tight text-stone-800 flex items-center gap-2">
          <div className="w-5 h-5 bg-stone-800 rounded-sm"></div>
          TaskFlow
        </div>
        <div className="flex items-center gap-4 text-sm text-stone-600">
          {headerContent}
          <div className="text-xs bg-stone-100 px-2 py-1 rounded border border-stone-200">Test Mode</div>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 max-w-5xl w-full mx-auto p-6 md:p-8">
        {children}
      </main>

      {/* Footer / Status Bar */}
      <footer className="h-8 border-t border-stone-200 bg-stone-100 flex items-center justify-between px-4 text-xs text-stone-500">
        <div className="flex items-center gap-4">
          <span>Ready</span>
          <span>Last sync: Never</span>
        </div>
        <div className="flex items-center gap-2">
          <span>Cmd+Shift+Space to capture</span>
        </div>
      </footer>
    </div>
  );
}
