FROM debian:7

MAINTAINER Fernando Almeida <fernando@fernandoalmeida.net>

RUN apt-get update -qq \
  && apt-get install -q --yes wget curl unzip git openjdk-7-jre

RUN echo "deb http://pkg.jenkins-ci.org/debian binary/" \
    > /etc/apt/sources.list.d/jenkins.list \
  && wget -q -O - http://pkg.jenkins-ci.org/debian/jenkins-ci.org.key \
    | apt-key add - \
  && apt-get update -qq \
  && apt-get install --yes -q jenkins

ENV JENKINS_HOME=/app

WORKDIR $JENKINS_HOME

RUN mkdir -p support plugins

COPY support plugins.txt $JENKINS_HOME/support/

RUN support/download_plugins.sh support/plugins.txt $JENKINS_HOME/plugins

ENTRYPOINT java -jar /usr/share/jenkins/jenkins.war
