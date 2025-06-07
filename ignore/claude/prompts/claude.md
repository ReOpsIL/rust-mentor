
## App Idea Clarification and Enhancement

**Purpose:** Transform a vague concept into a focused, compelling app idea with clear scope and value proposition.

**Prompt Template:**
```
I have a basic app idea in file @file:idea.md

Please enhance and refine this idea by:
1. Clarifying the core concept and making it more focused
2. Defining the target audience (demographics, behaviors, needs)
3. Identifying the specific problem it solves and why it matters
4. Articulating the main value proposition and unique selling points
5. Suggesting an appropriate scope that's achievable yet impactful
6. Highlighting what makes this app different from existing solutions
7. Providing a compelling 2-3 sentence elevator pitch

Format your response as a structured document with clear sections for each element above.
```

**Expected Output:** Enhanced app concept with clear target audience, problem statement, value proposition, and scope definition.

---

## Prompt 2: Feature Specification and Description

**Purpose:** Generate a comprehensive feature list that supports the app's core functionality and user goals.

**Prompt Template:**
```
Based on this enhanced app @file:detailed_app_idea.md 

Generate a comprehensive feature specification document that includes:

1. **Core Features** (3-5 essential features that define the app's primary functionality)
2. **Supporting Features** (5-8 features that enhance user experience and engagement)
3. **Advanced Features** (3-5 features for future releases or premium tiers)

For each feature, provide:
- Clear title and concise description
- User benefit and purpose explanation
- How it supports the overall app goals
- Priority level (High/Medium/Low)
- Estimated complexity (Simple/Moderate/Complex)
- Dependencies on other features

Also include:
- Feature interaction map showing how features work together
- User flow considerations for key features
- Potential feature conflicts or redundancies to avoid
```

**Expected Output:** Structured feature specification with prioritized lists, dependencies, and interaction mapping.

---

## Prompt 3: Design Specification Document

**Purpose:** Create detailed design guidelines covering UI/UX, user personas, journeys, and interface structure.

**Prompt Template:**
```
Using this app concept and feature list in project documents @file:detailed_app_idea.md , @file:features.md

Create a comprehensive design specification document including:

1. **User Personas** (2-3 detailed personas with demographics, goals, pain points, and tech comfort levels)

2. **User Journey Maps** (for each persona, map their journey from discovery to regular use)

3. **Information Architecture** (site map, navigation hierarchy, content organization)

4. **Key Screen Definitions** (describe 8-12 primary screens with their purpose and main elements)

5. **Navigation Flow** (how users move between screens, including entry/exit points)

6. **UI/UX Design Principles** (specific to this app's goals and user needs)

7. **Visual Design Guidelines** (color scheme suggestions, typography, iconography, spacing)

8. **Interaction Patterns** (buttons, forms, gestures, transitions, feedback mechanisms)

9. **Responsive Design Considerations** (how the app adapts to different screen sizes)

10. **Accessibility Requirements** (keyboard navigation, screen readers, color contrast)

Provide detailed textual descriptions for wireframes of the 3 most important screens.
```

**Expected Output:** Complete design specification with personas, journeys, screen definitions, and design guidelines.

---

## Prompt 4: Implementation Plan

**Purpose:** Develop a step-by-step implementation strategy with technology choices, phases, and team structure.

**Prompt Template:**
```
Based on this app project documents  @file:ux_specification.md, @file:detailed_app_idea.md , @file:features.md

Create a detailed implementation plan covering:

1. **Technology Stack Recommendations**
   - Frontend framework/library with justification
   - Backend technology and framework
   - Database solution(s)
   - Essential third-party services or APIs
   - Development and deployment tools

2. **Project Structure**
   - Repository organization
   - Module/component breakdown
   - Folder structure recommendations
   - Configuration management approach

3. **Development Phases** (Break into 4-6 phases with clear milestones)
   - Phase objectives and deliverables
   - Feature implementation priority
   - Timeline estimates
   - Success criteria for each phase

4. **Team Structure and Responsibilities**
   - Required roles and skills
   - Task allocation recommendations
   - Communication and collaboration tools

5. **Testing Strategy**
   - Unit testing approach
   - Integration testing plan
   - User acceptance testing criteria
   - Performance testing requirements

6. **Risk Assessment**
   - Technical risks and mitigation strategies
   - Timeline risks and contingency plans
   - Resource constraints and solutions

7. **Quality Assurance Process**
   - Code review standards
   - Documentation requirements
   - Performance benchmarks
```

**Expected Output:** Comprehensive implementation roadmap with technology stack, phases, team structure, and risk management.

---

## Prompt 5: System Architecture Overview

**Purpose:** Define the technical architecture with component relationships and create a visual system diagram.

**Prompt Template:**
```
Using @file:implementation_plan.md, @file:ux_specification.md, @file:detailed_app_idea.md , @file:features.md 

Generate a detailed system architecture document including:

1. **Architecture Overview**
   - High-level system description
   - Architectural pattern choice (MVC, microservices, etc.) with reasoning
   - Scalability considerations

2. **System Components**
   - Frontend application structure
   - Backend services and APIs
   - Database design overview
   - External service integrations
   - Security layer components

3. **Data Flow Description**
   - How data moves through the system
   - Request/response cycles
   - Data processing and transformation points
   - Caching strategies

4. **Technology Integration**
   - How chosen technologies work together
   - API design principles
   - Communication protocols
   - Error handling and logging approach

5. **Performance Considerations**
   - Load balancing strategies
   - Database optimization approaches
   - Caching mechanisms
   - CDN usage recommendations

6. **Security Architecture**
   - Authentication and authorization flow
   - Data encryption strategies
   - API security measures
   - Privacy protection mechanisms

7. **Mermaid.js System Diagram**
   Create a comprehensive system architecture diagram showing:
   - All major components
   - Data flow between components
   - External service connections
   - User interaction points

Please provide the Mermaid diagram code in a separate code block.
```

**Expected Output:** Technical architecture document with detailed component descriptions and a Mermaid system diagram.

---

## Prompt 6: Integration Strategy

**Purpose:** Define how all system components work together, including APIs, authentication, and third-party services.

**Prompt Template:**
```
Based on this complete system architecture:
and according to @file:architecture.md, @file:implementation_plan.md, @file:ux_specification.md, @file:detailed_app_idea.md , @file:features.md


Create a comprehensive integration strategy document covering:

1. **API Design and Contracts**
   - RESTful API endpoint specifications
   - Request/response formats and examples
   - Error response standards
   - API versioning strategy
   - Rate limiting and throttling

2. **Authentication and Authorization Integration**
   - User authentication flow
   - Session management
   - Role-based access control
   - Security token handling
   - Single sign-on considerations

3. **Data Integration**
   - Database connection strategies
   - Data synchronization between components
   - Data validation and sanitization
   - Backup and recovery integration

4. **Third-Party Service Integration**
   - External API connections and configurations
   - Webhook implementations
   - Service monitoring and health checks
   - Fallback strategies for service failures

5. **Frontend-Backend Integration**
   - State management between frontend and backend
   - Real-time communication (WebSockets, SSE)
   - File upload/download handling
   - Progressive web app considerations

6. **Testing Integration Points**
   - API testing strategies
   - Integration test scenarios
   - Mock service implementations
   - End-to-end testing approach

7. **CI/CD Integration Overview**
   - Automated testing pipeline
   - Deployment automation
   - Environment promotion strategies
   - Rollback procedures

8. **Monitoring and Logging Integration**
   - Application performance monitoring
   - Error tracking and alerting
   - User analytics integration
   - System health monitoring

9. **Development Workflow Integration**
   - Git branching strategy
   - Code review process
   - Documentation generation
   - Version control integration
```

**Expected Output:** Detailed integration strategy with API specifications, authentication flows, and development workflows.

---

## Prompt 7: Deployment, Installation, and Usage Documentation

**Purpose:** Create comprehensive documentation for building, deploying, and using the application.

**Prompt Template:**
```
Using all project documentation @file:integration.md @file:architecture.md, @file:implementation_plan.md, @file:ux_specification.md, @file:detailed_app_idea.md , @file:features.md


Generate complete deployment and usage documentation including:

1. **Development Environment Setup**
   - Prerequisites and system requirements
   - Step-by-step installation instructions
   - Environment configuration
   - Database setup and seeding
   - Local development server setup

2. **Build and Packaging**
   - Build process documentation
   - Configuration management
   - Asset optimization and bundling
   - Environment-specific builds
   - Dependency management

3. **Deployment Strategies**
   - Local deployment instructions
   - Staging environment setup
   - Production deployment process
   - Container deployment (if applicable)
   - Cloud deployment options

4. **Configuration Management**
   - Environment variables documentation
   - Configuration file templates
   - Secret management
   - Feature flag configuration
   - Third-party service configurations

5. **Installation Guide**
   - End-user installation instructions
   - System requirements for users
   - Installation troubleshooting
   - Initial setup and configuration
   - Account creation and onboarding

6. **User Guide and Documentation**
   - Getting started tutorial
   - Feature usage instructions
   - Common workflows and use cases
   - Troubleshooting guide
   - FAQ section

7. **Administrator Guide**
   - System administration tasks
   - User management
   - Data management and backup
   - Performance monitoring
   - Security maintenance

8. **Maintenance and Updates**
   - Regular maintenance tasks
   - Update and upgrade procedures
   - Database migration strategies
   - Backup and recovery procedures
   - Performance optimization

9. **Support and Community**
   - Support channels and resources
   - Community guidelines
   - Contribution guidelines
   - Issue reporting process
   - Documentation improvement process

10. **Appendices**
    - Glossary of terms
    - API reference summary
    - Configuration reference
    - Troubleshooting checklist
    - Resource links and references
```

**Expected Output:** Complete deployment and usage documentation covering all aspects from development to end-user support.

---



### Security Plan Prompt
```
Based on the complete app documents:
@file:integration.md @file:architecture.md, @file:implementation_plan.md, @file:ux_specification.md, @file:detailed_app_idea.md , @file:features.md

Create a comprehensive security plan including:
1. Threat modeling and risk assessment
2. Data protection and privacy measures
3. Secure coding practices and guidelines
4. Authentication and authorization security
5. API security measures
6. Data encryption strategies
7. Security testing and auditing procedures
8. Incident response procedures
9. Compliance requirements (GDPR, CCPA, etc.)
10. Security monitoring and logging
```

### Accessibility Guide Prompt
```
Based on the complete app documents:
@file:integration.md @file:architecture.md, @file:implementation_plan.md, @file:ux_specification.md, @file:detailed_app_idea.md , @file:features.md

Develop a comprehensive accessibility guide covering:
1. WCAG 2.1 compliance strategy
2. Keyboard navigation implementation
3. Screen reader compatibility
4. Color contrast and visual design accessibility
5. Mobile accessibility considerations
6. Accessibility testing procedures
7. User testing with disabled users
8. Accessibility documentation and training
9. Ongoing accessibility maintenance
10. Legal compliance requirements
```

### Localization Strategy Prompt
```
Based on the complete app documents:
@file:integration.md @file:architecture.md, @file:implementation_plan.md, @file:ux_specification.md, @file:detailed_app_idea.md , @file:features.md

Create a detailed localization strategy including:
1. Target markets and languages
2. Internationalization (i18n) technical implementation
3. Localization (L10n) process and workflow
4. Cultural adaptation considerations
5. RTL language support
6. Date, time, and currency formatting
7. Content management for multiple languages
8. Localization testing procedures
9. Translation management tools and processes
10. Maintenance and updates for localized versions
```

