# **FreeRadical Brand Design Specification v1.0**

## **1\. Logo Construction (The Hex-Bolt)**

The logo must maintain a mathematical relationship between the outer container and the internal energy source.

* **The Hexagon:** A regular hexagon rotated 0° (flat top). Represents the "Gear" of the Rust community.  
* **The Bolt:** A 3-point lightning strike. The points of the bolt should align with the 210° and 30° internal angles of the hexagon to create a sense of integrated energy.  
* **Clear Space:** Maintain a "safe zone" around the logo equal to 25% of the hexagon's total width.  
* **Minimum Size:** 24px (Digital) | 0.5" (Print). At smaller sizes, the accent "Data Nodes" should be removed for legibility.

## **2\. Color System**

The palette is designed to look "Industrial-Futuristic"—dark, authoritative backgrounds with high-vibrancy "Reactive" accents.

| Role | Name | Hex | Usage |
| :---- | :---- | :---- | :---- |
| **Primary** | Reactive Orange | \#F97316 | Action buttons, Logo Bolt, Active states. |
| **Foundation** | Deep Slate | \#0F172A | Primary backgrounds, Typography, Main Hexagon. |
| **Surface** | Carbon Gray | \#1E293B | Card backgrounds, Sidebar surfaces. |
| **Success** | Rust Cyan | \#06B6D4 | AI-features, Technical highlights, Positive ROI. |
| **Border** | Alloy Steel | \#334155 | Dividers, subtle borders, inactive inputs. |

## **3\. Typography**

* **Headings:** *Satoshi (Variable)*. Use **Black (900)** weight for headers with \-0.04em tracking. This creates a "heavy infrastructure" feel.  
* **Body:** *Inter*. Use **Medium (500)** for primary content. It provides high legibility for technical data.  
* **Technical:** *JetBrains Mono*. Used for all data-heavy metrics (LCP times, TTFB, Rust snippets).

## **4\. Component Standards**

* **Corners:** Use a 12px radius for standard components and 24px for large containers. We avoid "full round" pill shapes to maintain a more "engineered" look.  
* **Gradients:** Use sparingly. When used, the gradient should flow from Reactive Orange to a deeper Burnt Sienna.  
* **Shadows:** Use "Hard Shadows"—minimal blur, high offset (e.g., 8px 8px 0px rgba(15, 23, 42, 0.1)).

## **5\. Iconography**

* **Style:** 2px stroke weight, geometric shapes, non-rounded terminals.  
* **Grid:** All icons should be built on a 24x24 pixel grid.  
* **Interactive State:** Icons should transition from Alloy Steel to Reactive Orange on hover.