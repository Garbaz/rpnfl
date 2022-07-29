import Relation.Binary.PropositionalEquality as Eq
open Eq using (_≡_; refl)
open import Data.Empty using (⊥; ⊥-elim)
open import Data.Nat using (ℕ; zero; suc; _<_; _≤?_; z≤n; s≤s)
open import Relation.Nullary using (¬_)
open import Relation.Nullary.Decidable using (True; toWitness)
open import Function using (id; _∘_)

infix  4 _⊢_
infix  4 _∋_↝_
infixl 5 _▷_↝_
infixr 7 _⇒_

-- infix  5 ƛ_
-- infix  5 μ_
-- infixl 7 _·_
-- infix  8 `suc_
-- infix  9 `_
-- infix  9 S_
-- infix  9 #_

data Type : Set where
  _⇒_ : Type → Type → Type
  𝓝  : Type


data Context : Set where
  ∅   : Context
  _▷_↝_ : Context → Type → Type → Context

data _∋_↝_ : Context → Type → Type → Set where

  `零 : ∀ {Γ A B}
      ---------
    → Γ ▷ A ↝ B ∋ A ↝ B

  `继_ : ∀ {Γ A B C D}
    → Γ ∋ A ↝ B
      ---------
    → Γ ▷ C ↝ D ∋ A ↝ B

data _⊢_ : Context → Type → Set where
    $_ : ∀ {Γ A B} → Γ ∋ A ↝ B → Γ ⊢ A
    %_ : ∀ {Γ A B} → Γ ∋ A ↝ B → Γ ⊢ A ⇒ B
    零 : ∀ {Γ} → Γ ⊢ 𝓝
    继_ : ∀ {Γ} → Γ ⊢ 𝓝 → Γ ⊢ 𝓝
    配 : ∀ {Γ A} → Γ ⊢ 𝓝 → Γ ⊢ A → Γ ▷ 𝓝 ↝ A ⊢ A → Γ ⊢ A
    〔_〕 : ∀ {Γ A B} → Γ ▷ A ↝ B ⊢ A ⇒ B → Γ ⊢ A ⇒ B

