
## Post-MVP / Future Features

### **Feature 8: User Accounts & Persistent Progress Tracking**
*   **Feature ID:** F008
*   **Description:** Users can create simple accounts (locally stored or cloud-based) to log in. The application saves their learning progress, topics covered, exercise attempts/successes, and skill level preferences across multiple sessions.
*   **Purpose & User Benefit:** Allows users to track their learning journey over time, resume where they left off, and see their improvement. Solves the problem of losing context between sessions and helps maintain motivation.
*   **Support for App Goals:** Would significantly enhance "Personalized & Adaptive Learning" and "Maintaining Momentum."
*   **Key Interactions/Dependencies:** Would interact with F002 (storing/retrieving level), F004 (tracking completed content).
*   **Priority:** Should-have (Post-MVP)

### **Feature 9: In-App Code Execution & Basic Exercise Validation**
*   **Feature ID:** F009
*   **Description:** Users can attempt to write Rust code for exercises directly within a simple editor in the TUI. The application could then attempt to compile and run this code (potentially in a sandboxed environment) and provide basic feedback (e.g., compile errors, or simple pass/fail for predefined tests if feasible).
*   **Purpose & User Benefit:** Provides immediate feedback on practice, reinforcing learning more effectively and helping users identify misunderstandings quickly. Greatly enhances the "Interactive Reinforcement" aspect.
*   **Support for App Goals:** Would significantly improve the "Interactive Reinforcement" value proposition and make the "Integrated Learning Loop" more complete and effective.
*   **Key Interactions/Dependencies:** Would interact with F004 (exercises) and F001 (TUI for code input/output).
*   **Priority:** Should-have (Post-MVP)

### **Feature 10: Advanced Natural Language User Queries**
*   **Feature ID:** F010
*   **Description:** Users can ask free-form questions about Rust concepts (e.g., "Explain borrowing in more detail," "What's the difference between `String` and `&str`?") or request specific topics using natural language, and the AI Mentor responds appropriately.
*   **Purpose & User Benefit:** Offers a more flexible, conversational, and interactive learning experience, allowing users to clarify doubts or explore specific areas of interest on demand, much like a real mentor.
*   **Support for App Goals:** Would significantly enhance "Personalized & Adaptive Learning" and make the interaction more powerful.
*   **Key Interactions/Dependencies:** Would heavily rely on F003 (LLM Integration) and F004 (content generation capabilities), requiring more sophisticated prompt engineering.
*   **Priority:** Could-have (Post-MVP)

### **Feature 11: Gamification Elements**
*   **Feature ID:** F011
*   **Description:** The app includes elements like points for completed topics/exercises, badges for achievements
 (e.g., "Mastered 5 borrowing exercises"), or daily streaks for consistent learning.
*   **Purpose & User Benefit:** Increases user engagement and motivation, potentially improving learning consistency and addressing "Maintaining Momentum" in a fun way.
*   **Support for App Goals:** Could support "Maintaining Momentum" and overall user retention.
*   **Key Interactions/Dependencies:** Would likely depend on F008 (User Accounts & Progress Tracking).
*   **Priority:** Could-have (Post-MVP)

### **Feature 12: Offline Functionality (Cached Content)**
*   **Feature ID:** F012
*   **Description:** The application caches a limited set of previously generated or popular learning modules for different levels.
 This allows users to access some content even when an internet connection is not available.
*   **Purpose & User Benefit:** Allows users to learn even without internet access, increasing accessibility and convenience.
*   **Support for App Goals:** Would improve accessibility.
*   **Key Interactions/Dependencies:** Would require changes to how content from F004 is stored and retrieved. F003 (LLM Integration) would not be available for new content generation in offline mode.
*   **Priority:** Could-have (Post-MVP)

### **Feature 13: TUI Customization Options**
*   **Feature ID:** F013
*   **Description:** Users can customize some aspects of the Terminal User Interface, such as color schemes or basic layout preferences.
*   **Purpose & User Benefit:** Improves user comfort and personalization of the learning environment, making the experience more enjoyable for prolonged use.
*   **Support for App Goals:** Enhances the "Distraction-Free TUI Experience" by allowing users to tailor it to their visual preferences.
*   **Key Interactions/Dependencies:** Would interact with F001 (Basic TUI Navigation & Display).
*   **Priority:** Could-have (Post-MVP)

### **Feature 14: Expanded Content Sourcing & Topic Coverage**
*   **Feature ID:** F014
*   **Description:** The LLM prompting strategy is expanded to include a wider range of Rust documentation and advanced topics,
 beyond the initial focus on "The Rust Programming Language" and "Rust by Example." This could involve more fine-grained control over sub-topics within the `std` library or advanced language features.
*   **Purpose & User Benefit:** Caters to more advanced learners and provides a more comprehensive learning tool covering a broader spectrum of Rust knowledge.
*   **Support for App Goals:** Enhances "Authoritative & Up-to-Date Knowledge" and makes the tool useful for a wider range of the target audience, including intermediate learners looking to explore new areas.
*   **Key Interactions/Dependencies:** Primarily an enhancement to the backend logic and prompting for F004 (AI-Powered Learning Module Generation).
*   **Priority:** Should-have (Post-MVP, for continuous improvement and broader appeal)

---
**Note on "Won't-have for MVP" items from the app idea:**
The features explicitly listed as "will NOT initially include" in the app idea document (User accounts, In-app code execution,
Advanced NLU, Gamification, Offline functionality, TUI customization, Coverage of highly advanced topics) are reflected above with
"Should-have (Post-MVP)" or "Could-have (Post-MVP)" priorities, indicating they are valuable but outside the initial MVP scope.