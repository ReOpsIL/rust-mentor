# Rust AI Mentor: Feature List

This document outlines the features for the Rust AI Mentor application, based on the enhanced app idea.

---

## Core MVP Features

### **Feature 1: Basic TUI Navigation & Display**
*   **Feature ID:** F001
*   **Description:** The user can launch the application in their terminal. The TUI (Terminal User Interface) provides a basic,
      keyboard-navigable interface to display initial prompts, learning content (topic, explanation, code, exercises), 
      and options for interaction.
*   **Purpose & User Benefit:** Provides the foundational interactive environment. It allows users to access and 
      interact with the app's functionalities directly within their terminal, offering a focused, distraction-free learning experience as
      per the app's core value.
*   **Support for App Goals:** Directly supports the "Core TUI" MVP requirement and the "Distraction-Free TUI Experience" value proposition.
      It's essential for delivering any content or interaction.
*   **Key Interactions/Dependencies:** Underpins all other interactive features (F002, F004, F005, F013).
*   **Priority:** Must-have

### **Feature 2: User Skill Level Selection**
*   **Feature ID:** F002
*   **Description:** Upon starting a new learning session, the user is prompted to select their current Rust skill level 
      (e.g., on a scale of 1 to 10). This selection is used for the current session to tailor the generated content.
*   **Purpose & User Benefit:** Enables personalized learning by ensuring the content delivered is appropriate for the
      user's current understanding. This prevents beginners from being overwhelmed and advanced users  
      from receiving overly simplistic material, directly addressing the "Information Overload" problem.
*   **Support for App Goals:** Directly supports the "Level Selection" MVP requirement and the
      "Personalized & Adaptive Learning" core value proposition.
*   **Key Interactions/Dependencies:** The selected level is a key input for F004 (AI-Powered Learning Module Generation).
      Depends on F001 for UI.
*   **Priority:** Must-have

### **Feature 2.5: Content Source Selection**
*   **Feature ID:** F002.5
*   **Description:** After selecting a skill level, the user is prompted to choose a content source for their learning session:
      - Rust Library - Standard and community libraries
      - Rust By Example - Examples from the Rust By Example guide
      - The Rust Programming Language - Topics from the official Rust book
      - Random - Randomly selects from any of the above sources
*   **Purpose & User Benefit:** Provides users with more control over their learning experience by allowing them to focus on specific
      aspects of Rust that interest them most. This enhances personalization and helps users target their learning goals more effectively.
*   **Support for App Goals:** Enhances the "Personalized & Adaptive Learning" core value proposition by adding another dimension of customization.
*   **Key Interactions/Dependencies:** Occurs after F002 (level selection) and before F004 (content generation). The selected source
      determines which data index will be used for topic selection.
*   **Priority:** Must-have

### **Feature 3: LLM Integration (OpenRouter)**
*   **Feature ID:** F003
*   **Description:** The application integrates with an LLM provider (initially OpenRouter) to send structured prompts
      and receive generated learning content. This includes handling API communication, request/response parsing,
      and necessary configurations (e.g., API key management, though the specifics of key management are an implementation detail).
*   **Purpose & User Benefit:** This feature is the engine for dynamic content generation. It allows the app to create lessons,
      examples, and exercises based on authoritative sources (as guided by prompts), ensuring content can be fresh and tailored.
*   **Support for App Goals:** Directly supports the "AI-Powered Content Generation" MVP requirement and the "Authoritative & Up-to-Date Knowledge" value proposition by enabling the use of an LLM.
*   **Key Interactions/Dependencies:** Essential for F004 (AI-Powered Learning Module Generation). Requires an internet connection.
*   **Priority:** Must-have

### **Feature 4: AI-Powered Learning Module Generation**
*   **Feature ID:** F004
*   **Description:** Based on the user's selected skill level (from F002) and content source, the application prompts the LLM (via F003) to:
    1. The application will use one of the following pre-created indices (in JSON format) based on user selection:
       - Rust Library Index - Standard and community libraries
       - Rust By Example - Examples from the Rust By Example guide
       - The Rust Programming Language - Topics from the official Rust book
       - Random - Randomly selects from any of the above sources
    2. Select a topic according to the user level selected.
    3. Generate a concise explanation (target 5-10 lines).
    4. Provide 1-3 illustrative code snippets for the topic.
    5. Create 1-3 small exercises related to the topic.
        The LLM is guided to base content on foundational resources appropriate to the selected source.
*   **Purpose & User Benefit:** Delivers the core learning experience. Users receive tailored, bite-sized lessons with practical examples
      and exercises, addressing "Information Overload," "Lack of Structured Practice," and "Maintaining Momentum" by providing clear,
      progressive steps.
*   **Support for App Goals:** Fulfills the main "AI-Powered Content Generation" requirements (topic, explanation, code, exercises)
      and supports "Personalized & Adaptive Learning," "Efficient & Focused Content," and "Interactive Reinforcement." 
      Aligns with the "Content Sourcing Strategy."
*   **Key Interactions/Dependencies:** Depends on F001 (for displaying content via F005), F002 (for level input), and F003 (for LLM interaction).
*   **Priority:** Must-have

### **Feature 5: Learning Content Display**
*   **Feature ID:** F005
*   **Description:** The TUI effectively presents the AI-generated learning module (topic, explanation, code snippets, and exercises from F004)
      to the user in a clear, readable, and well-structured format within the terminal - use syntax highlighting for code snippets - implement using off the shelf rust library.
*   **Purpose & User Benefit:** Ensures the user can easily consume and understand the learning materials provided by the AI. A clear presentation is crucial for an effective learning experience and maintaining focus.
*   **Support for App Goals:** Supports the "Core TUI" and "Distraction-Free TUI Experience." It's the means by which the value of F004 is delivered to the user.
*   **Key Interactions/Dependencies:** Depends on F001 (TUI framework) and F004 (receives content to display).
*   **Priority:** Must-have

### **Feature 6: Session-Based Learning (Stateless Operation)**
*   **Feature ID:** F006
*   **Description:** The application operates on a per-session basis. 
    When the user starts the app, they select a level and can request learning modules. 
    There is NO persistence of user progress, history, or settings between application runs for the MVP.
    Each launch of the application is a fresh start.
*   **Purpose & User Benefit:** Simplifies the MVP development by focusing on the core learning interaction loop.
      Users get immediate value in each session without the complexity of account management or long-term progress tracking.
*   **Support for App Goals:** Aligns with the "Stateless or Minimal State" MVP scope boundary. 
      It allows for rapid iteration on the core learning loop.
*   **Key Interactions/Dependencies:** This is a guiding principle for MVP features, ensuring they don't rely on persistent state.
*   **Priority:** Must-have (as a characteristic/constraint for MVP)

### **Feature 7: Request New Learning Module**
*   **Feature ID:** F007
*   **Description:** After viewing a learning module (topic, explanation, code, exercises), 
      the user has an option within the TUI to request a new, random, level-appropriate learning module.
*   **Purpose & User Benefit:** Enables continuous learning within a single session. 
      Users can explore multiple topics based on their selected skill level without restarting the application, 
      supporting "broader exploration and serendipitous learning." This forms the core "learning loop."
*   **Support for App Goals:** Directly supports "On-Demand, Level-Specific Topic Generation" and the concept of an
      "Integrated Learning Loop in TUI." Enhances user engagement by allowing them to consume more content per session.
*   **Key Interactions/Dependencies:** Depends on F001 (UI for the request option), F002 (to maintain level context for the new request),
      and triggers F004 (AI-Powered Learning Module Generation) for a new module.
*   **Priority:** Must-have
