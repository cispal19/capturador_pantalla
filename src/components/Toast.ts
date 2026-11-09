/**
 * Displays a subtle, non-intrusive floating toast notification.
 */
export function showToast(message: string, duration = 2500): HTMLElement {
  const toast = document.createElement("div");
  toast.className = "screenshot-toast";
  toast.textContent = message;
  toast.style.position = "fixed";
  toast.style.bottom = "24px";
  toast.style.left = "50%";
  toast.style.transform = "translateX(-50%)";
  toast.style.backgroundColor = "rgba(15, 23, 42, 0.9)";
  toast.style.color = "#ffffff";
  toast.style.padding = "8px 16px";
  toast.style.borderRadius = "8px";
  toast.style.fontSize = "14px";
  toast.style.fontWeight = "500";
  toast.style.boxShadow = "0 4px 12px rgba(0, 0, 0, 0.15)";
  toast.style.zIndex = "10000000";
  toast.style.pointerEvents = "none";
  toast.style.transition = "opacity 0.2s ease-in-out";

  document.body.appendChild(toast);

  setTimeout(() => {
    toast.style.opacity = "0";
    setTimeout(() => {
      if (toast.parentNode) {
        toast.parentNode.removeChild(toast);
      }
    }, 200);
  }, duration);

  return toast;
}
