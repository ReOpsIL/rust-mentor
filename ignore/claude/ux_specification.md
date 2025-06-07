# RustMentor Design Specification Document

## 1. User Personas

### Persona 1: "Sarah, The Career Changer"

**Demographics:**
- Age: 28
- Background: Frontend developer transitioning to systems programming
- Experience: 4 years JavaScript/TypeScript, 6 months exploring Rust
- Location: Remote worker, urban environment
- Education: Self-taught developer with bootcamp background

**Goals:**
- Master Rust fundamentals to transition into backend/systems role
- Build confidence with memory management and ownership concepts
- Complete structured learning in 3-4 months during evening hours
- Create portfolio projects demonstrating Rust proficiency

**Pain Points:**
- Overwhelmed by Rust's complexity compared to garbage-collected languages
- Limited time (1-2 hours/evening) requires efficient learning
- Struggles with borrowing/lifetime concepts
- Needs validation that she's progressing correctly

**Tech Comfort Level:**
- High: Web technologies, Git, basic CLI usage
- Medium: System administration, debugging
- Low: Memory management, low-level programming concepts

**Behavior Patterns:**
- Prefers visual progress indicators and structured paths
- Values immediate feedback and validation
- Learns best through hands-on practice with clear explanations
- Uses multiple monitors but focuses on single tasks

### Persona 2: "Marcus, The Systems Veteran"

**Demographics:**
- Age: 35
- Background: Senior C++ developer with 12+ years experience
- Current Role: Team lead at embedded systems company
- Location: On-site, tech hub city
- Education: Computer Science degree, continuous learner

**Goals:**
- Evaluate Rust as C++ alternative for new projects
- Understand Rust's safety guarantees and performance characteristics
- Learn idiomatic Rust patterns and ecosystem
- Make informed technology decisions for team

**Pain Points:**
- Limited time due to management responsibilities
- Needs to justify learning investment to leadership
- Skeptical of new languages without proven enterprise adoption
- Wants deep understanding, not surface-level tutorials

**Tech Comfort Level:**
- High: System programming, CLI tools, debugging, architecture
- High: Memory management, performance optimization
- Medium: Modern build systems and package managers
- Low: Functional programming paradigms

**Behavior Patterns:**
- Terminal-native workflow, keyboard shortcuts enthusiast
- Prefers comprehensive understanding over quick wins
- Values authoritative sources and detailed explanations
- Short, focused learning sessions between meetings

### Persona 3: "Alex, The CS Student"

**Demographics:**
- Age: 21
- Background: Computer Science junior specializing in systems programming
- Experience: Strong academic foundation, limited industry experience
- Location: University campus, shared study spaces
- Education: Formal CS curriculum with C, Java, Python background

**Goals:**
- Master Rust for senior capstone project
- Build competitive programming skills
- Prepare for technical interviews at systems companies
- Explore open-source contribution opportunities

**Pain Points:**
- Academic pressure with multiple concurrent projects
- Inconsistent internet connectivity in study locations
- Budget constraints limit access to premium learning resources
- Needs efficient study methods for exam preparation

**Tech Comfort Level:**
- High: Programming fundamentals, algorithms, data structures
- Medium: CLI usage, version control, debugging
- Medium: System concepts from coursework
- Low: Real-world development workflows and tools

**Behavior Patterns:**
- Intensive study sessions during available blocks
- Collaborative learning with study groups
- Documentation-heavy learning style
- Platform-agnostic tool preferences

## 2. User Journey Maps

### Sarah's Journey: Discovery to Mastery

**Stage 1: Discovery (Week 1)**
- **Trigger:** Sees Rust mentioned in job postings for desired roles
- **Actions:** Researches Rust learning options, finds RustMentor via Reddit/GitHub
- **Touchpoints:** GitHub README, initial app download, first launch
- **Emotions:** Curious but intimidated by Rust's reputation
- **Needs:** Clear getting-started guidance, reassurance about difficulty

**Stage 2: Initial Exploration (Weeks 1-2)**
- **Actions:** Completes level 1-2 exercises, explores documentation browser
- **Touchpoints:** Welcome tutorial, basic syntax exercises, progress tracking
- **Emotions:** Encouraged by structured approach, frustrated by ownership concepts
- **Needs:** Patient explanations, immediate feedback, clear progress indicators

**Stage 3: Skill Building (Weeks 3-8)**
- **Actions:** Regular evening sessions, progresses through levels 3-6
- **Touchpoints:** AI-generated exercises, spaced repetition reviews, topic modules
- **Emotions:** Growing confidence, occasional overwhelm with complex topics
- **Needs:** Consistent challenge level, review of difficult concepts, motivation

**Stage 4: Application (Weeks 9-12)**
- **Actions:** Tackles advanced topics (levels 7-9), starts personal project
- **Touchpoints:** Code execution environment, note exports, analytics dashboard
- **Emotions:** Excited about capabilities, confident in fundamentals
- **Needs:** Real-world application guidance, portfolio development support

**Stage 5: Mastery & Advocacy (Month 4+)**
- **Actions:** Completes learning path, contributes to open source, recommends tool
- **Touchpoints:** Community features, advanced exercises, external project integration
- **Emotions:** Accomplished, eager to share knowledge
- **Needs:** Continuous learning opportunities, community connection

### Marcus's Journey: Evaluation to Integration

**Stage 1: Research (Week 1)**
- **Trigger:** Management asks for Rust evaluation for next-generation products
- **Actions:** Seeks authoritative, time-efficient learning resource
- **Touchpoints:** Professional networks, GitHub search, tool evaluation
- **Emotions:** Skeptical but professionally obligated
- **Needs:** Credible source validation, efficient time investment

**Stage 2: Focused Learning (Weeks 2-4)**
- **Actions:** Concentrated sessions on core concepts, compares to C++ patterns
- **Touchpoints:** Documentation browser, advanced exercises, performance insights
- **Emotions:** Impressed by safety features, concerned about adoption complexity
- **Needs:** Deep technical details, performance comparisons, enterprise readiness

**Stage 3: Team Preparation (Weeks 5-6)**
- **Actions:** Creates learning plan for team, identifies migration strategies
- **Touchpoints:** Content export features, learning analytics, curriculum planning
- **Emotions:** Confident in recommendation, excited about team development
- **Needs:** Training resources for team, implementation guidance

**Stage 4: Implementation (Month 2+)**
- **Actions:** Leads team through Rust adoption, refers to advanced topics
- **Touchpoints:** Ongoing reference usage, team learning coordination
- **Emotions:** Validated in technology choice, committed to mastery
- **Needs:** Advanced topics, team progress tracking, continuous updates

### Alex's Journey: Academic Project to Career Preparation

**Stage 1: Assignment Research (Week 1)**
- **Trigger:** Professor assigns Rust-based systems programming project
- **Actions:** Researches learning resources, prioritizes free options
- **Touchpoints:** Academic recommendations, student forums, free tool search
- **Emotions:** Excited about new language, concerned about learning curve
- **Needs:** Free access, comprehensive coverage, academic credibility

**Stage 2: Intensive Learning (Weeks 2-4)**
- **Actions:** Daily study sessions, progresses rapidly through fundamentals
- **Touchpoints:** Offline mode, progressive levels, exercise validation
- **Emotions:** Motivated by progress, stressed about project deadlines
- **Needs:** Reliable offline access, efficient learning path, quick mastery

**Stage 3: Project Development (Weeks 5-8)**
- **Actions:** Applies learned concepts to capstone project, seeks help
- **Touchpoints:** Code execution environment, documentation reference, problem-solving
- **Emotions:** Confident in basics, challenged by integration complexity
- **Needs:** Real-world application guidance, debugging support, best practices

**Stage 4: Career Preparation (Semester end+)**
- **Actions:** Adds Rust skills to resume, prepares for technical interviews
- **Touchpoints:** Portfolio development, interview preparation, skill validation
- **Emotions:** Proud of accomplishment, ready for career opportunities
- **Needs:** Skill certification, interview preparation, industry relevance

## 3. Information Architecture

### Site Map Structure

```
RustMentor/
├── Dashboard
│   ├── Progress Overview
│   ├── Recent Activity
│   ├── Quick Actions
│   └── Learning Stats
├── Learning Path
│   ├── Level Selection (1-10)
│   ├── Topic Categories
│   │   ├── Fundamentals
│   │   ├── Ownership & Borrowing
│   │   ├── Error Handling
│   │   ├── Concurrency
│   │   └── Advanced Topics
│   └── Custom Learning Plans
├── Documentation Browser
│   ├── Search Interface
│   ├── Navigation Tree
│   ├── Content Viewer
│   └── Bookmarks
├── Practice Arena
│   ├── AI-Generated Exercises
│   ├── Code Execution Environment
│   ├── Exercise History
│   └── Performance Analytics
├── Review System
│   ├── Spaced Repetition Queue
│   ├── Weak Areas Focus
│   ├── Review Analytics
│   └── Custom Review Sets
├── Progress & Analytics
│   ├── Learning Dashboard
│   ├── Skill Assessment
│   ├── Time Tracking
│   └── Export Reports
├── Settings & Preferences
│   ├── Theme Customization
│   ├── Learning Preferences
│   ├── Notification Settings
│   └── Data Export/Import
└── Help & Support
    ├── Quick Start Guide
    ├── Feature Documentation
    ├── Troubleshooting
    └── Community Resources
```


### Navigation Hierarchy

**Primary Navigation (Always Visible):**
1. Dashboard (Home)
2. Learn (Learning Path)
3. Browse (Documentation)
4. Practice (Exercises)
5. Review (Spaced Repetition)
6. Progress (Analytics)

**Secondary Navigation (Context-Dependent):**
- Topic-specific sub-menus
- Exercise difficulty filters
- Documentation sections
- Search and filter controls

**Tertiary Navigation (Detail Views):**
- Breadcrumb navigation
- Related content suggestions
- Cross-references and links
- Action buttons and shortcuts

### Content Organization Principles

**By Learning Progression:**
- Linear path through difficulty levels
- Prerequisite relationships clearly defined
- Optional advanced topics branching from core path

**By Topic Area:**
- Conceptual groupings align with Rust learning model
- Cross-cutting concerns (testing, debugging) integrated throughout
- Real-world application examples in each category

**By User Intent:**
- Quick reference vs. deep learning modes
- Structured curriculum vs. exploratory browsing
- Practice-focused vs. theory-focused content

## 4. Key Screen Definitions

### Screen 1: Dashboard (Home Screen)
**Purpose:** Central hub providing overview of learning progress and quick access to key features
**Main Elements:**
- Welcome message with current learning streak
- Progress indicators for overall completion and current level
- Quick action buttons (Continue Learning, Start Practice, Browse Docs)
- Recent activity feed showing completed exercises and milestones
- Learning goals and next recommended actions
- Daily learning time tracker

### Screen 2: Learning Path Navigator
**Purpose:** Structured progression through Rust concepts with clear difficulty scaling
**Main Elements:**
- Visual level progression map (1-10) with completion status
- Topic category selection (Fundamentals, Ownership, Concurrency, etc.)
- Current position indicator and next suggested lesson
- Estimated time commitments for each section
- Prerequisites and dependencies clearly marked
- Alternative learning path options based on experience level

### Screen 3: Documentation Browser
**Purpose:** Interactive interface for exploring official Rust documentation
**Main Elements:**
- Hierarchical navigation tree with expandable sections
- Main content viewer with syntax highlighting
- Search bar with auto-complete and filtering options
- Bookmark and note-taking functionality
- Related topics and cross-reference links
- Integration buttons to generate exercises from current content

### Screen 4: AI Exercise Generator
**Purpose:** Dynamic practice environment with contextually relevant coding challenges
**Main Elements:**
- Exercise prompt and requirements display
- Code editor with syntax highlighting and error detection
- AI difficulty adjustment controls
- Hint system with progressive revelation
- Solution validation and feedback display
- Exercise history and performance tracking

### Screen 5: Code Execution Environment
**Purpose:** Integrated Rust compiler and runtime for testing code examples
**Main Elements:**
- Split-pane layout: editor and output terminal
- Compilation status and error messages
- Code templates and snippet insertion
- Debug output and variable inspection
- Performance metrics (compilation time, execution time)
- Save and export functionality for working code

### Screen 6: Progress Analytics Dashboard
**Purpose:** Comprehensive view of learning progress with actionable insights
**Main Elements:**
- Overall progress charts and completion percentages
- Skill strength radar chart showing competency areas
- Time investment tracking and learning velocity
- Weak areas identification with recommended focus topics
- Achievement badges and milestone celebrations
- Comparative progress against learning goals

### Screen 7: Spaced Repetition Review
**Purpose:** Algorithm-driven review system for reinforcing learned concepts
**Main Elements:**
- Review queue with prioritized concepts
- Confidence rating interface for self-assessment
- Concept explanation refresh before review
- Adaptive scheduling based on performance
- Review statistics and retention metrics
- Custom review set creation and management

### Screen 8: Settings & Customization
**Purpose:** Personalization options for optimal learning experience
**Main Elements:**
- Theme selection and color customization
- Learning preference configuration (pace, difficulty, focus areas)
- Notification and reminder settings
- Data export/import options
- Integration settings for external tools
- Account and progress backup options

## 5. Navigation Flow

### Primary Navigation Patterns

**Linear Learning Flow:**
Dashboard → Learning Path → Topic Selection → Lesson Content → Practice Exercise → Progress Update → Next Lesson

**Exploratory Browse Flow:**
Dashboard → Documentation Browser → Search/Navigate → Content View → Generate Exercise → Practice → Return to Documentation

**Review Session Flow:**
Dashboard → Review System → Spaced Repetition Queue → Concept Review → Assessment → Performance Update → Continue/Exit

**Deep Dive Flow:**
Any Screen → Search → Documentation → Related Topics → Advanced Exercises → Code Execution → Note Export

### Entry Points
1. **Application Launch:** Always opens to Dashboard
2. **Deep Links:** Direct access to specific topics or exercises
3. **Search Results:** Context-sensitive entry into relevant content
4. **Notifications:** Guided entry for review sessions or new content

### Exit Points
1. **Natural Completion:** Lesson or exercise completion with progress save
2. **Pause Points:** Mid-session saves with easy resume functionality
3. **Quick Exit:** Emergency exit with auto-save and session restoration
4. **Export Actions:** Content extraction with return to original context

### Inter-Screen Transitions
- **Contextual Back Navigation:** Intelligent back button preserving user flow
- **Cross-References:** Seamless jumping between related concepts
- **Progressive Disclosure:** Step-by-step revelation of complex topics
- **Breadcrumb Navigation:** Clear path tracking for complex navigation trees

## 6. UI/UX Design Principles

### Core Design Philosophy
**Terminal-Native Excellence:** Embrace the power and efficiency of terminal interfaces while providing modern UX conveniences

**Focused Learning:** Minimize cognitive load through clean, distraction-free interfaces that support deep concentration

**Adaptive Intelligence:** Leverage AI to personalize the experience without overwhelming users with unnecessary complexity

### Specific Design Principles

**1. Progressive Disclosure**
- Present information in digestible chunks
- Reveal complexity gradually as user expertise grows
- Provide depth on demand without cluttering primary interfaces

**2. Immediate Feedback**
- Real-time validation of code and concepts
- Clear progress indicators and achievement recognition
- Responsive interface updates that confirm user actions

**3. Contextual Assistance**
- Help and hints available exactly when needed
- Smart suggestions based on current learning context
- Unobtrusive guidance that doesn't interrupt flow

**4. Consistency & Predictability**
- Standardized navigation patterns across all screens
- Consistent terminology and interaction models
- Reliable keyboard shortcuts and muscle memory support

**5. Error Recovery & Resilience**
- Graceful handling of mistakes with learning opportunities
- Auto-save functionality to prevent progress loss
- Clear error messages with actionable resolution steps

**6. Cognitive Load Management**
- Single-focus interfaces that avoid overwhelming choices
- Smart defaults that work for most users
- Optional complexity for advanced users

## 7. Visual Design Guidelines

### Color Scheme Recommendations

**Primary Palette (Terminal-Inspired):**
- **Background:** Deep charcoal (#1e1e1e) for reduced eye strain
- **Primary Text:** Light gray (#d4d4d4) for optimal readability
- **Accent Primary:** Rust orange (#ce422b) for branding and highlights
- **Accent Secondary:** Steel blue (#569cd6) for interactive elements
- **Success:** Forest green (#4ec9b0) for completion and positive feedback
- **Warning:** Amber (#d7ba7d) for caution and attention
- **Error:** Coral red (#f44747) for errors and critical issues

**Alternative Themes:**
- **High Contrast:** Pure black background with white text
- **Light Mode:** Cream background with dark text for daylight use
- **Custom:** User-configurable color combinations

### Typography Guidelines

**Primary Font:** JetBrains Mono or system monospace
- **Rationale:** Designed for developers, excellent code readability
- **Fallbacks:** Fira Code, Source Code Pro, system monospace

**Font Sizes:**
- **Headers:** 16pt (relative to terminal base size)
- **Body Text:** 14pt (standard terminal size)
- **Code:** 14pt (consistent with body for readability)
- **Captions:** 12pt (for supplementary information)

**Font Weights:**
- **Regular (400):** Primary text content
- **Medium (500):** Section headers and emphasis
- **Bold (700):** Important headings and alerts

### Iconography System

**Style:** Minimal, geometric icons optimized for terminal display
**Set:** Custom icon font based on developer-familiar symbols
**Examples:**
- Navigation: Arrows, chevrons, hamburger menu
- Actions: Play/pause, save, export, search
- Status: Check marks, warnings, progress indicators
- Content: Code brackets, documentation pages, exercise symbols

### Spacing & Layout

**Grid System:** 8px base unit for consistent spacing
- **Micro Spacing:** 4px (tight element spacing)
- **Standard Spacing:** 8px (default element margins)
- **Section Spacing:** 16px (between major sections)
- **Screen Spacing:** 24px (page margins and large separations)

**Layout Principles:**
- **Responsive Columns:** Adapt to terminal width with minimum viable widths
- **Consistent Margins:** Standardized spacing throughout application
- **Visual Hierarchy:** Clear distinction between content levels
- **Breathing Room:** Adequate white space to prevent visual cluttering

## 8. Interaction Patterns

### Button Design & Behavior
**Primary Actions:** Bold outline with accent color, keyboard shortcuts visible
**Secondary Actions:** Subtle outline with muted colors
**Destructive Actions:** Red accent with confirmation requirements
**Disabled States:** Grayed out with clear indication of why disabled

### Form Interactions
**Input Fields:** Clear focus indicators with validation feedback
**Selection Controls:** Radio buttons and checkboxes optimized for keyboard navigation
**Dropdowns:** Searchable selections with keyboard navigation support
**Validation:** Real-time feedback with helpful error messages

### Gesture Support (Future Mobile Considerations)
**Swipe Navigation:** Horizontal swipes for moving between lessons
**Pull to Refresh:** Update content and sync progress
**Pinch to Zoom:** Adjust text size for code viewing
**Long Press:** Context menu access for advanced options

### Transition Animations
**Screen Transitions:** Smooth sliding animations respecting terminal limitations
**Content Loading:** Subtle progress indicators without distraction
**State Changes:** Micro-animations for feedback and confirmation
**Performance:** Optimized for terminal rendering capabilities

### Feedback Mechanisms
**Visual Feedback:** Color changes, highlighting, progress indicators
**Textual Feedback:** Clear status messages and confirmations
**Progressive Feedback:** Real-time validation and suggestion updates
**Achievement Feedback:** Celebration animations for milestones

## 9. Responsive Design Considerations

### Terminal Size Adaptations

**Minimum Viable Size:** 80x24 characters (classic terminal standard)
**Optimal Size:** 120x40 characters (modern development standard)
**Large Display:** 160x60+ characters (ultra-wide monitor support)

### Layout Strategies

**Mobile-First Approach (for eventual mobile version):**
- Single-column layouts that stack gracefully
- Touch-friendly button sizes and spacing
- Simplified navigation appropriate for smaller screens

**Desktop Optimization:**
- Multi-pane layouts for efficient information density
- Keyboard-first interaction patterns
- Optimal use of available screen real estate

### Content Adaptation

**Text Scaling:** Responsive font sizes based on terminal capabilities
**Information Density:** Progressive disclosure based on available space
**Navigation Adaptation:** Collapsible menus and context-sensitive controls
**Code Display:** Horizontal scrolling with line wrapping options

### Performance Considerations

**Rendering Optimization:** Efficient terminal drawing for smooth scrolling
**Memory Management:** Lazy loading of content for large documentation sets
**Bandwidth Awareness:** Offline-first design with smart caching
**Battery Impact:** Minimal CPU usage for mobile device compatibility

## 10. Accessibility Requirements

### Keyboard Navigation
**Tab Order:** Logical flow through all interactive elements
**Keyboard Shortcuts:** Comprehensive shortcuts for all major actions
**Focus Indicators:** Clear visual indication of current focus
**Escape Patterns:** Consistent escape key behavior for modal dismissal

### Screen Reader Support
**Semantic Structure:** Proper heading hierarchy and landmark regions
**Alt Text:** Descriptive text for all visual elements and icons
**Live Regions:** Dynamic content updates announced to screen readers
**Context Information:** Clear description of current location and available actions

### Color & Contrast
**WCAG AA Compliance:** Minimum 4.5:1 contrast ratio for normal text
**Color Independence:** Information conveyed through multiple channels, not just color
**High Contrast Mode:** Alternative theme with maximum contrast ratios
**Color Blindness:** Testing with various color blindness simulations

### Cognitive Accessibility
**Clear Language:** Simple, direct instructions and feedback
**Consistent Navigation:** Predictable interface patterns throughout
**Error Prevention:** Validation and confirmation for destructive actions
**Memory Support:** Clear indication of current state and progress

### Assistive Technology Compatibility
**Screen Reader Testing:** Verification with NVDA, JAWS, and VoiceOver
**Voice Control:** Support for Dragon NaturallySpeaking and similar tools
**Switch Navigation:** Compatibility with assistive hardware devices
**Motor Impairment:** Adjustable timing and alternative input methods

## 11. Detailed Wireframe Descriptions

### Wireframe 1: Dashboard (Most Important Screen)

**Layout Structure:**
```
┌─ RustMentor Dashboard ────────────────────────────────────────────────┐
│ [≡] RustMentor                    [⚙] Settings    [?] Help    [×] Exit │
├───────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  Welcome back, Sarah! 🦀          Learning Streak: 7 days            │
│                                                                       │
│  ┌─ Current Progress ─────────────────┐  ┌─ Quick Actions ──────────┐ │
│  │                                    │  │                          │ │
│  │  Level 4: Ownership & Borrowing    │  │  [Continue Learning]     │ │
│  │  ████████████░░░░░░░ 65%           │  │  [Practice Exercises]    │ │
│  │                                    │  │  [Browse Documentation] │ │
│  │  Overall: ████████░░░ 43%          │  │  [Review Session]        │ │
│  │                                    │  │                          │ │
│  │  Next: Understanding Lifetimes     │  │  [View Progress]         │ │
│  │  Est. Time: 25 minutes             │  │                          │ │
│  │                                    │  │                          │ │
│  └────────────────────────────────────┘  └──────────────────────────┘ │
│                                                                       │
│  ┌─ Recent Activity ──────────────────────────────────────────────────┐ │
│  │                                                                    │ │
│  │  ✓ Completed: Basic Ownership Concepts (2 hours ago)              │ │
│  │  ✓ Mastered: 8/10 Borrowing Exercises (Yesterday)                 │ │
│  │  📚 Bookmarked: "Common Lifetime Patterns" (2 days ago)           │ │
│  │  🏆 Achievement: "Ownership Explorer" unlocked (3 days ago)       │ │
│  │                                                                    │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                       │
│  ┌─ Today's Focus ──────────────────┐  ┌─ Learning Stats ───────────┐ │
│  │                                  │  │                            │ │
│  │  🎯 Recommended: Lifetime Basics │  │  Time Today: 45 min        │ │
│  │  📝 Review: 3 concepts due       │  │  This Week: 4.2 hours      │ │
│  │  💪 Challenge: Advanced exercise │  │  Concepts Mastered: 23     │ │
│  │                                  │  │  Exercises Completed: 67   │ │
│  └──────────────────────────────────┘  └────────────────────────────┘ │
│                                                                       │
├───────────────────────────────────────────────────────────────────────┤
│ [Tab] Navigate  [Enter] Select  [Ctrl+L] Learning Path  [Ctrl+D] Docs │
└───────────────────────────────────────────────────────────────────────┘
```


**Key Interactive Elements:**
- **Navigation Header:** Always visible with consistent menu access
- **Progress Cards:** Clickable sections leading to detailed progress views
- **Quick Action Buttons:** Large, prominent buttons for primary user flows
- **Activity Feed:** Scrollable history with clickable items for context
- **Keyboard Shortcuts:** Visible shortcuts for power users

**Visual Hierarchy:**
- **Primary:** Current progress and next actions dominate visual space
- **Secondary:** Recent activity provides context and motivation
- **Tertiary:** Statistics and recommendations support decision-making

### Wireframe 2: AI Exercise Generator (Second Most Important)

**Layout Structure:**
```
┌─ Practice Arena: AI-Generated Exercise ───────────────────────────────┐
│ [←] Back to Learning   Topic: Ownership   Level: 4   Difficulty: ●●●○ │
├───────────────────────────────────────────────────────────────────────┤
│                                                                       │
│  ┌─ Exercise Prompt ──────────────────────────────────────────────────┐ │
│  │                                                                    │ │
│  │  Challenge: Fixing Ownership Violations                           │ │
│  │                                                                    │ │
│  │  The following code has ownership issues that prevent compilation. │ │
│  │  Fix the errors while maintaining the intended functionality.      │ │
│  │                                                                    │ │
│  │  💡 Hint: Consider when variables are moved vs. borrowed          │ │
│  │  🎯 Goal: Make the code compile and pass all tests                │ │
│  │                                                                    │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                       │
│  ┌─ Code Editor ─────────────────────┐ ┌─ Output & Feedback ─────────┐ │
│  │                                   │ │                             │ │
│  │  fn main() {                      │ │  💥 Compilation Error:      │ │
│  │      let data = vec![1, 2, 3];    │ │                             │ │
│  │      process_data(data);          │ │  error[E0382]: borrow of    │ │
│  │      println!("{:?}", data);      │ │  moved value: `data`        │ │
│  │  }                                │ │                             │ │
│  │                                   │ │  Line 4: `data` was moved  │ │
│  │  fn process_data(mut v: Vec<i32>) │ │  in line 3, cannot be used │ │
│  │  {                                │ │  again                      │ │
│  │      v.push(4);                   │ │                             │ │
│  │      println!("Processed: {:?}",  │ │  [Run Code] [Get Hint]      │ │
│  │                v);                │ │  [Check Solution]           │ │
│  │  }                                │ │                             │ │
│  │                                   │ │                             │ │
│  └───────────────────────────────────┘ └─────────────────────────────┘ │
│                                                                       │
│  ┌─ Exercise Tools ───────────────────────────────────────────────────┐ │
│  │                                                                    │ │
│  │  [Reset Code] [Load Template] [Export Solution] [Skip Exercise]    │ │
│  │                                                                    │ │
│  │  Progress: 3/5 test cases passing    Time: 8m 23s                 │ │
│  │  Attempts: 2/∞ (no penalty)         Hints used: 1/3               │ │
│  │                                                                    │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                       │
├───────────────────────────────────────────────────────────────────────┤
│ [Ctrl+R] Run  [Ctrl+H] Hint  [Ctrl+S] Save  [Tab] Switch Pane        │
└───────────────────────────────────────────────────────────────────────┘
```


**Key Interactive Elements:**
- **Code Editor:** Syntax-highlighted editor with error indicators
- **Real-time Feedback:** Immediate compilation results and error explanation
- **Progressive Hints:** Graduated assistance without giving away solutions
- **Tool Actions:** Quick access to common exercise operations

**Learning Support Features:**
- **Contextual Error Explanation:** Links errors to learning concepts
- **Progress Tracking:** Visual indication of completion status
- **Adaptive Difficulty:** System adjusts based on user performance

### Wireframe 3: Documentation Browser (Third Most Important)

**Layout Structure:**
```
┌─ Rust Documentation Browser ──────────────────────────────────────────┐
│ [←] Back   [🔍] Search: "lifetime"        [📖] Bookmarks   [⚙] Settings │
├──────────────────┬────────────────────────────────────────────────────┤
│                  │                                                    │
│ ┌─ Navigation ─┐ │ ┌─ Content: Understanding Lifetimes ─────────────┐ │
│ │              │ │ │                                                │ │
│ │ 📘 The Book  │ │ │  # Understanding Lifetimes                    │ │
│ │ ├─ Getting    │ │ │                                                │ │
│ │ │  Started    │ │ │  Lifetimes are Rust's way of ensuring that    │ │
│ │ ├─ Ownership  │ │ │  references are valid for as long as we need  │ │
│ │ │  ├─ Basics  │ │ │  them to be.                                  │ │
│ │ │  ├─ Borrow. │ │ │                                                │ │
│ │ │  └─[Lifetm.]│ │ │  ## The Main Aim of Lifetimes                 │ │
│ │ ├─ Structs    │ │ │                                                │ │
│ │ └─ Error Hand.│ │ │  The main aim of lifetimes is to prevent      │ │
│ │              │ │ │  dangling references, which cause a program    │ │
│ │ 📖 Reference │ │ │  to reference data other than the data it's    │ │
│ │ ├─ Keywords   │ │ │  intended to reference.                       │ │
│ │ ├─ Syntax     │ │ │                                                │ │
│ │ └─ Std Lib    │ │ │  ```rust                                      │ │
│ │              │ │ │  {                                             │ │
│ │ 🎯 By Example │ │ │      let r;                                    │ │
│ │ ├─ Hello      │ │ │      {                                         │ │
│ │ ├─ Primitives │ │ │          let x = 5;                           │ │
│ │ └─ Variables  │ │ │          r = &x;                              │ │
│ │              │ │ │      }                                         │ │
│ └──────────────┘ │ │      println!("r: {}", r);                    │ │
│                  │ │  }                                             │ │
│ Search Results:  │ │  ```                                           │ │
│ ┌──────────────┐ │ │                                                │ │
│ │ ● Lifetime   │ │ │  💡 This code won't compile because `x` goes  │ │
│ │   Syntax     │ │ │     out of scope before `r` tries to use it.  │ │
│ │ ● Function   │ │ │                                                │ │
│ │   Lifetimes  │ │ │  [📝 Generate Exercise] [🔖 Bookmark]          │ │
│ │ ● Struct     │ │ │  [📤 Export Notes] [🔗 Related Topics]        │ │
│ │   Lifetimes  │ │ │                                                │ │
│ └──────────────┘ │ └────────────────────────────────────────────────┘ │
│                  │                                                    │
├──────────────────┴────────────────────────────────────────────────────┤
│ [Ctrl+F] Search  [Ctrl+B] Bookmark  [Ctrl+E] Exercise  [↑↓] Navigate │
└───────────────────────────────────────────────────────────────────────┘
```


**Key Interactive Elements:**
- **Expandable Navigation Tree:** Hierarchical content with clear current location
- **Search Integration:** Real-time search with results preview
- **Content Actions:** Direct integration with exercise generation and note-taking
- **Cross-References:** Clickable links to related concepts

**Learning Integration Features:**
- **Exercise Generation:** Create practice problems from documentation content
- **Progress Integration:** Track which documentation sections have been studied
- **Personalization:** Bookmarking and note-taking for future reference

These wireframes demonstrate the core user flows and establish the visual and interaction patterns that will be consistent throughout the application. The terminal-native design prioritizes keyboard navigation while providing modern UX conveniences that support effective learning.