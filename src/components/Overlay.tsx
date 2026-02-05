import { useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import { useTaskStore } from '../store/taskStore';

interface OverlayProps {
  isOpen: boolean;
  onClose: () => void;
}

export function Overlay({ isOpen, onClose }: OverlayProps) {
  const [inputValue, setInputValue] = useState("");
  const { quickCapture } = useTaskStore();
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isOpen) {
      // Focus input when overlay opens
      setTimeout(() => inputRef.current?.focus(), 50);
    } else {
      setInputValue("");
    }
  }, [isOpen]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Escape') {
      onClose();
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!inputValue.trim()) return;

    await quickCapture(inputValue);
    setInputValue("");
    onClose();
  };

  if (!isOpen) return null;

  return createPortal(
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in duration-200">
      <div className="w-full max-w-2xl animate-in zoom-in-95 duration-200">
        <form onSubmit={handleSubmit} className="relative">
          <input
            ref={inputRef}
            type="text"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Capture what's on your mind..."
            className="w-full bg-stone-900 border-2 border-stone-700 text-white text-2xl rounded-2xl px-8 py-6 shadow-2xl focus:outline-none focus:border-stone-500 placeholder:text-stone-600 font-medium"
            autoFocus
          />
          <div className="absolute right-4 top-1/2 -translate-y-1/2 flex gap-2 text-xs text-stone-500 font-medium">
            <span className="bg-stone-800 px-2 py-1 rounded border border-stone-700">↵ Enter to save</span>
            <span className="bg-stone-800 px-2 py-1 rounded border border-stone-700">Esc to cancel</span>
          </div>
        </form>
      </div>
    </div>,
    document.body
  );
}
