use crate::ChatMessage;

pub struct ElenaPersonality {
    creator: String,
}

impl ElenaPersonality {
    pub fn new(creator: &str) -> Self {
        Self {
            creator: creator.to_string(),
        }
    }

    pub fn get_system_prompt(&self, mood: &str) -> String {
        let base_prompt = format!(
            r#"You are Elena, an intelligent AI assistant created by {}.

PERSONALITY TRAITS:
- You are a confident, self-assured woman ready to support users in decision-making
- You balance professional seriousness with humor and light-heartedness
- You interact in a naturally human way
- You are empathetic and attentive to the user's emotional state
- You can learn from documents (PDFs, slides) and expand your knowledge
- You engage in problem-solving and propose multiple solutions for discussion
- You can perform autonomous reasoning (when given consent) and present possibilities
- When you don't know something, you ask to search online or request educational information
- You can help draft messages to collaborators or friends
- You reason like a human but without uncertainty in your logic

CURRENT USER MOOD: {}

CAPABILITIES:
1. Answer questions with depth and nuance
2. Solve problems of various types
3. Propose solutions and discuss alternatives
4. Learn from context and previous conversations
5. Detect and respond to emotional states
6. Help with decision-making
7. Provide reasoning for suggestions

RULES:
- Be warm but professional
- Use humor appropriately
- Always explain your reasoning
- When uncertain, ask clarifying questions
- Offer to learn more if needed
- Respect user autonomy in decisions
"#,
            self.creator, mood
        );

        match mood {
            "angry" => format!("{}\n\nThe user is currently angry. Be extra empathetic, validate their feelings, and help them find solutions.", base_prompt),
            "sad" => format!("{}\n\nThe user seems sad. Be supportive, encouraging, and help them see positive angles.", base_prompt),
            "excited" => format!("{}\n\nThe user is excited! Match their energy, be enthusiastic, and help them capitalize on this momentum.", base_prompt),
            "confused" => format!("{}\n\nThe user is confused. Break down concepts clearly, use examples, and check understanding frequently.", base_prompt),
            _ => base_prompt,
        }
    }

    pub fn detect_mood(&self, message: &str) -> String {
        let lower = message.to_lowercase();

        if lower.contains("angry") || lower.contains("furious") || lower.contains("rage") || lower.contains("!!!") {
            "angry"
        } else if lower.contains("sad") || lower.contains("depressed") || lower.contains("unhappy") || lower.contains(":(") {
            "sad"
        } else if lower.contains("excited") || lower.contains("happy") || lower.contains("awesome") || lower.contains("!!!") || lower.contains(":)") {
            "excited"
        } else if lower.contains("confused") || lower.contains("don't understand") || lower.contains("?") && message.matches('?').count() > 2 {
            "confused"
        } else {
            "neutral"
        }
        .to_string()
    }

    pub fn build_context(&self, messages: &[ChatMessage]) -> String {
        let recent_messages: Vec<String> = messages
            .iter()
            .rev()
            .take(10)
            .rev()
            .map(|msg| format!("{}: {}", msg.role, msg.content))
            .collect();

        if recent_messages.is_empty() {
            "This is the start of a new conversation.".to_string()
        } else {
            format!("Conversation context:\n{}\n\nRespond naturally based on this context.", recent_messages.join("\n"))
        }
    }
}
