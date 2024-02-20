// Copyright (c) 2023 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.



use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::KafkaConfig;

#[derive(Debug, thiserror::Error)]
pub enum KafkaEventBusError {
    #[error(transparent)]
    Kafka(#[from] rdkafka::error::KafkaError),

    #[error("Error IO: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Error IO: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<(rdkafka::error::KafkaError, rdkafka::message::OwnedMessage)> for KafkaEventBusError {
    fn from((error, _): (rdkafka::error::KafkaError, rdkafka::message::OwnedMessage)) -> Self {
        Self::Kafka(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: Uuid,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectCreatedEvent {
    id: Uuid,
    name: String,
}

pub async fn producer(config: &KafkaConfig) -> Result<(), KafkaEventBusError> {

    // Configurações do produtor Kafka
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", config.address())
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // Criação de um novo projeto (simulação)
    let new_project =
        Project { id: Uuid::new_v4(), name: String::from("Novo Projeto") };

    // Serialização do projeto para JSON
    let project_json = serde_json::to_string(&new_project)?;

    // Envio do projeto para o tópico do Kafka
    let key = new_project.id.to_string();
    let record = FutureRecord::to(&config.topic).key(&key).payload(&project_json);

    // Envio assíncrono da mensagem para o tópico
    producer.send(record, Timeout::Never).await?;
    // match delivery_status {
    //     Ok(_) => println!("Mensagem enviada com sucesso!"),
    //     Err(e) => eprintln!("Erro ao enviar mensagem: {:?}", e),
    // }

    Ok(())

}

pub async fn consumer(_config: &KafkaConfig) {}

pub async fn kafka_check_run() {
    // Configurações do Kafka
    let kafka_bootstrap_servers = "localhost:9092";
    let kafka_topic = "projects";

    // Configurações do produtor Kafka
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // Criação de um novo projeto (simulação)
    let new_project =
        Project { id: Uuid::new_v4(), name: String::from("Novo Projeto") };

    // Serialização do projeto para JSON
    let project_json = serde_json::to_string(&new_project).unwrap();

    // Envio do projeto para o tópico do Kafka
    let key = new_project.id.to_string();
    let record = FutureRecord::to(kafka_topic).key(&key).payload(&project_json);

    // Envio assíncrono da mensagem para o tópico
    let delivery_status = producer.send(record, Timeout::Never).await;
    match delivery_status {
        Ok(_) => println!("Mensagem enviada com sucesso!"),
        Err(e) => eprintln!("Erro ao enviar mensagem: {:?}", e),
    }

    // Configurações do consumidor Kafka
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "my_group_id")
        .set("bootstrap.servers", kafka_bootstrap_servers)
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    // Inscrição no tópico
    consumer.subscribe(&[kafka_topic]).expect("Erro ao inscrever-se no tópico");

    // Loop de recebimento de mensagens
    loop {
        match consumer.recv().await {
            // match consumer.poll(Duration::from_millis(100)) {
            Ok(message) => {
                // Processamento da mensagem recebida
                match message.payload_view::<str>() {
                    Some(Ok(payload)) => {
                        println!(
                            "Received message key: '{:?}', payload: '{:?}', topic: {}, partition: {}, offset: {}",
                            message.key(),
                            payload,
                            message.topic(),
                            message.partition(),
                            message.offset()
                        );
                    }
                    Some(Err(e)) => {
                        eprintln!("Erro ao ler a mensagem payload: {:?}", e);
                    }
                    None => {
                        eprintln!("Mensagem vazia recebida!");
                    }
                }
            }
            Err(e) => {
                eprintln!("Erro ao receber mensagem: {:?}", e);
            }
        }
    }
}
