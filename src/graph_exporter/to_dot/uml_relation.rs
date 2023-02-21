use super::{HasDotEntity, DotEntity};
use crate::uml_entity::*;
use dot_graph::{Edge, Style, Arrow, ArrowShape, Fill, Side};

impl HasDotEntity for UMLRelation {
    fn get_dot_entities(&self, name_prefix: &str) -> Vec<DotEntity> {
        let from: String;
        let to: String;

        // ugly impletation, should be removed once refactoring done
        // TODO
        if !self.from.contains(".") {
            from = vec![name_prefix, &self.from].concat();
        } else {
            from = self.from.clone();
        }
        if !self.to.contains(".") {
            to = vec![name_prefix, &self.to].concat();
        } else {
            to = self.to.clone();
        }
        match self.kind {
            UMLRelationKind::UMLAggregation => {
                vec![DotEntity::Edge(Edge::new(
                    &from, 
                    &to, 
                    "")
                    .start_arrow(Arrow::from_arrow(ArrowShape::Diamond(Fill::Open, Side::Both))),
                )]
            },
            UMLRelationKind::UMLComposition => {
                vec![DotEntity::Edge(Edge::new(
                    &from, 
                    &to, 
                    "")
                    .end_arrow(Arrow::from_arrow(ArrowShape::diamond()))
                )]
            },
            UMLRelationKind::UMLDependency => {
                vec![DotEntity::Edge(Edge::new(
                    &from,
                    &to, 
                    "")
                    .style(Style::Dashed)
                    .end_arrow(Arrow::from_arrow(ArrowShape::vee()))
                )]
            },
            UMLRelationKind::UMLAssociationUni => {
                vec![DotEntity::Edge(Edge::new(
                    &from,
                    &to,
                    "")
                    .end_arrow(Arrow::from_arrow(ArrowShape::vee()))
                )]
            },
            UMLRelationKind::UMLAssociationBi => {
                vec![DotEntity::Edge(Edge::new(
                    &from, 
                    &to, 
                    "")
                    .end_arrow(Arrow::none())
                )]
            },
            UMLRelationKind::UMLRealization => {
                vec![DotEntity::Edge(Edge::new(
                    &from, 
                    &to, 
                    "")
                    .end_arrow(Arrow::from_arrow(ArrowShape::Normal(Fill::Open, Side::Both)))
                    .style(Style::Dashed),
                )]
            },
        }
    }
}