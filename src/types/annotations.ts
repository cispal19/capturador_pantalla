export type ActiveTool =
  | "select"
  | "rectangle"
  | "ellipse"
  | "arrow"
  | "line"
  | "freehand"
  | "text"
  | "highlighter"
  | "obfuscation";

export type CorporateColor =
  | "#ef4444" // Red
  | "#10b981" // Green
  | "#3b82f6" // Blue
  | "#f59e0b" // Yellow
  | "#ffffff" // White
  | "#000000"; // Black

export type StrokeThickness = 2 | 4 | 8; // Fine, Medium, Thick

export type AnnotationItem =
  | {
      type: "rectangle";
      id: string;
      x: number;
      y: number;
      width: number;
      height: number;
      color: string;
      stroke_width: number;
    }
  | {
      type: "ellipse";
      id: string;
      x: number;
      y: number;
      radius_x: number;
      radius_y: number;
      color: string;
      stroke_width: number;
    }
  | {
      type: "arrow";
      id: string;
      start_x: number;
      start_y: number;
      end_x: number;
      end_y: number;
      color: string;
      stroke_width: number;
    }
  | {
      type: "line";
      id: string;
      start_x: number;
      start_y: number;
      end_x: number;
      end_y: number;
      color: string;
      stroke_width: number;
    }
  | {
      type: "freehand";
      id: string;
      points: Array<[number, number]>;
      color: string;
      stroke_width: number;
    }
  | {
      type: "text";
      id: string;
      x: number;
      y: number;
      content: string;
      font_size: number;
      color: string;
    }
  | {
      type: "highlighter";
      id: string;
      points: Array<[number, number]>;
      color: string;
      stroke_width: number;
    }
  | {
      type: "obfuscation";
      id: string;
      x: number;
      y: number;
      width: number;
      height: number;
      block_size: number;
    };

export const CORPORATE_PALETTE: CorporateColor[] = [
  "#ef4444",
  "#10b981",
  "#3b82f6",
  "#f59e0b",
  "#ffffff",
  "#000000",
];
